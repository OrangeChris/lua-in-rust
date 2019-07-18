//! An `Object` is some data which:
//! - Has an unknown lifetime
//! - May have references to other `Object`s
//!
//! Because of this, it needs to be garbage collected.
#![allow(dead_code)]

use std::cell::Cell;
use std::collections::HashSet;
use std::ops::{Deref, DerefMut, Drop};
use std::ptr::{self, NonNull};

use crate::Table;

/// A wrapper around the `LuaVal`s which need to be garbage-collected.
pub struct Object {
    /// The value this object holds.
    pub raw: RawObject,
    /// The next object in the heap.
    next: *mut Object,
    /// A flag used in garbage-collection. This is behind a `Cell` so that
    /// we can alter the keys of a table.
    color: Cell<Color>,
}

pub enum RawObject {
    Table(Table),
}

impl RawObject {
    pub fn type_string(&self) -> &str {
        match self {
            RawObject::Table(_) => "table",
        }
    }
}

/// The internal pointer type objects use to point to each other.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ObjectPtr(NonNull<Object>);

impl Deref for ObjectPtr {
    type Target = Object;
    fn deref(&self) -> &Object {
        unsafe { self.0.as_ref() }
    }
}

impl DerefMut for ObjectPtr {
    fn deref_mut(&mut self) -> &mut Object {
        unsafe { self.0.as_mut() }
    }
}

#[derive(Clone, Copy)]
enum Color {
    Unmarked,
    Reachable,
}

/// A collection of objects which need to be garbage-collected.
pub struct GcHeap {
    start: *mut Object,
    size: usize,
    old_size: usize,
    roots: HashSet<ObjectPtr>,
}

impl GcHeap {
    pub fn new_table(&mut self) -> ObjectPtr {
        let table = Table::default();
        let new_object = Object {
            next: self.start,
            color: Cell::new(Color::Unmarked),
            raw: RawObject::Table(table),
        };
        let boxed = Box::new(new_object);
        let raw_ptr = Box::into_raw(boxed);
        // Pointers from Box::into_raw are guaranteed to not be null.
        let obj_ptr = ObjectPtr(NonNull::new(raw_ptr).unwrap());

        self.start = raw_ptr;
        self.size += 1;

        obj_ptr
    }

    /// Run the garbage-collector
    pub fn collect(&mut self) {
        self.mark();
        self.sweep();
    }

    /// Walk through every object in the heap, starting with the roots.
    fn mark(&mut self) {
        for root in &self.roots {
            root.mark_reachable();
        }
    }

    fn sweep(&mut self) {
        self.old_size = self.size;
        let mut next_ptr_ref = &mut self.start;
        while !next_ptr_ref.is_null() {
            // From right-to-left, this unsafe block means:
            // - deref the reference (safe) to get a pointer
            // - deref the pointer (unsafe) to get an Object
            // - make a mutable reference to that Object
            let next_obj = unsafe { &mut **next_ptr_ref };
            match next_obj.color.get() {
                Color::Reachable => {
                    // Reset its color.
                    next_obj.color.set(Color::Unmarked);
                    next_ptr_ref = &mut next_obj.next;
                }
                Color::Unmarked => {
                    let boxed = unsafe { Box::from_raw(*next_ptr_ref) };
                    *next_ptr_ref = boxed.next;
                    self.size -= 1;
                }
            }
        }
    }
}

impl Default for GcHeap {
    fn default() -> Self {
        Self {
            start: ptr::null_mut(),
            size: 0,
            old_size: 0,
            roots: HashSet::default(),
        }
    }
}

impl Drop for GcHeap {
    fn drop(&mut self) {
        let mut next_ptr = self.start;
        while !next_ptr.is_null() {
            let boxed = unsafe { Box::from_raw(next_ptr) };
            next_ptr = boxed.next;
            // Now the boxed object is dropped.
        }
    }
}

pub trait Markable {
    /// Mark this item and the references it contains as reachable.
    fn mark_reachable(&self);
}

impl Markable for Object {
    fn mark_reachable(&self) {
        if let Color::Unmarked = self.color.get() {
            self.color.set(Color::Reachable);
            self.raw.mark_reachable();
        }
    }
}

impl Markable for RawObject {
    fn mark_reachable(&self) {
        match self {
            RawObject::Table(tbl) => tbl.mark_reachable(),
        }
    }
}