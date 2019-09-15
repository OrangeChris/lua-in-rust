use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ErrorKind {
    BadNumber,
    Complexity,
    InvalidCharacter,
    Io(io::Error),
    TooManyLocals,
    TooManyNumbers,
    TooManyStrings,
    UnclosedString,
    UnexpectedEof,
    UnexpectedTok,
    UnsupportedFeature,
    TableKeyNan,
    TableKeyNil,
    TypeError,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    line_num: usize,
    column: usize,
}

impl ErrorKind {
    pub fn is_recoverable(&self) -> bool {
        match self {
            ErrorKind::UnclosedString | ErrorKind::UnexpectedEof | ErrorKind::UnexpectedTok => true,
            _ => false,
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;
        match self {
            BadNumber => write!(f, "malformed number"),
            Complexity => write!(f, "complexity"),
            InvalidCharacter => write!(f, "invalid character"),
            Io(e) => write!(f, "{}", e),
            TooManyLocals => write!(f, "too many local variables"),
            TooManyNumbers => write!(f, "too many literal numbers"),
            TooManyStrings => write!(f, "too many literal strings"),
            UnclosedString => write!(f, "unfinished string"),
            UnexpectedEof => write!(f, "unexpected <eof>"),
            UnexpectedTok => write!(f, "syntax error"),
            UnsupportedFeature => write!(f, "unsupported feature"),
            TableKeyNan => write!(f, "table index was NaN"),
            TableKeyNil => write!(f, "table index was nil"),
            TypeError => write!(f, "type error"),
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind, line_num: usize, column: usize) -> Self {
        Error {
            kind,
            line_num,
            column,
        }
    }

    pub fn without_location(kind: ErrorKind) -> Self {
        Error::new(kind, 0, 0)
    }

    pub fn from_io_error(io_error: io::Error) -> Self {
        let kind = ErrorKind::Io(io_error);
        Error::without_location(kind)
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn line_num(&self) -> usize {
        self.line_num
    }

    pub fn is_recoverable(&self) -> bool {
        self.kind.is_recoverable()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error {}:{}: {}", self.line_num, self.column, self.kind)
    }
}
