#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Instr {
    BranchTrue(usize),
    BranchFalse(usize),
    Pop,

    Print,
    Assign,
    GlobalLookup,

    PushNil,
    PushBool(bool),
    PushNum(usize),
    PushString(usize),

    // binary operators
    Add,
    Subtract,
    Multiply,
    Divide,
    Pow,
    Mod,
    Concat,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,

    // unary
    Not,
    Length,
    Negate,
}
