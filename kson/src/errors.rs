use backtrace::Backtrace;
use bytes::Bytes;
use std::{fmt, sync::Arc};

pub type KsonError = Arc<Error>;

#[derive(Debug, Clone)]
pub struct Error {
    pub backtrace: Backtrace,
    pub location: Location,
    pub message: Option<String>,
    pub bytes: Bytes,
    pub offset: usize,
    pub variant: Variant,
}

#[derive(Debug, Clone)]
pub enum Variant {
    LengthError { expected: usize, remaining: usize },
    WrongType { expected: crate::Type, found: u8 },
    IntTooShort { tag_len: u8, max_len: u8 },
}

use Variant::*;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error while deserializing at {}", self.location)?;
        if let Some(msg) = &self.message {
            write!(f, " with extra msg {}", msg)?;
        }
        write!(
            f,
            "\n\
             Raw bytes were: {:#?}\n\
             Error found at offset: {}\n\
             Variant was: {}\n\
             Backtrace was:\n\
             {:#?}
             ",
            self.bytes,
            self.offset,
            (match self.variant {
                LengthError {
                    expected,
                    remaining,
                } => format!(
                    "expected {} bytes but only {} remained",
                    expected, remaining
                ),
                WrongType { expected, found } => {
                    format!("expected type {:?} but only found {:x?}", expected, found)
                }
                IntTooShort { tag_len, max_len } => format!(
                    "tried to deserialize int of length {} as type with max len {}",
                    tag_len, max_len
                ),
            }),
            self.backtrace
        )
    }
}

#[derive(Debug, Clone, Copy)]
/// A location in source code
pub struct Location {
    /// The line where the error occurred
    pub line: u32,
    /// The column where the error occurred
    pub col: u32,
    /// The file where the error occurred
    pub file: &'static str,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{file}:{line}:{column}",
            file = self.file,
            line = self.line,
            column = self.file
        )
    }
}

#[macro_export]
/// Returns the location this macro was called from
macro_rules! loc {
    () => {
        $crate::errors::Location {
            file: file!(),
            line: line!(),
            col: column!(),
        }
    };
}

#[macro_export]
macro_rules! E {
    ($var: expr, $byt: expr, $offset: expr, $($t: tt),*) => {
        ::std::sync::Arc::new($crate::errors::Error {
            backtrace: ::backtrace::Backtrace::new(),
            location: $crate::loc!(),
            bytes: $byt,
            offset: $offset,
            variant: {
                use $crate::errors::Variant::*;
                $var
            },
            message: Some(format!($($t),*))
        })
    };

    ($var: expr, $byt: expr, $offset: expr) => {
        ::std::sync::Arc::new($crate::errors::Error {
            backtrace: ::backtrace::Backtrace::new(),
            location: $crate::loc!(),
            bytes: $byt,
            offset: $offset,
            variant: {
                use $crate::errors::Variant::*;
                $var
            },
            message: None,
        })
    };
}

#[macro_export]
macro_rules! e {
    ($var: expr, $byt: expr, $offset: expr, $($t:tt),*) => {
        Err::<(), $crate::errors::KsonError>(E!($var, $byt, $offset, $($t),*))?
    };

    ($var: expr, $byt: expr, $offset: expr) => {
        Err::<(), $crate::errors::KsonError>(E!($var, $byt, $offset))?
    };
}
