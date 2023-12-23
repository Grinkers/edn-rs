use alloc::boxed::Box;
use core::fmt::{self, Debug, Display};
use core::{convert, num, str};

pub struct Error {
    pub(crate) code: Code,
    pub(crate) line: Option<usize>,
    pub(crate) column: Option<usize>,
}

#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Code {
    /// Catchall/placeholder error messages
    Message(Box<str>),

    /// Parse errors
    InvalidChar,
    InvalidEscape,
    InvalidKeyword,
    InvalidRadix(Option<u8>),
    ParseNumber(ParseNumber),
    UnexpectedEOF,
    UnmatchedDelimiter(char),

    // Feature errors
    NoFeatureSets,

    // Deserialize errors
    Convert(&'static str),

    // Navigation errors
    Iter,

    /// For type conversions
    TryFromInt(num::TryFromIntError),
    #[doc(hidden)]
    Infallable(), // Makes the compiler happy for converting u64 to u64 and i64 to i64
}

#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ParseNumber {
    ParseIntError(num::ParseIntError),
    ParseFloatError(num::ParseFloatError),
}

impl Error {
    pub(crate) const fn deserialize(conv_type: &'static str) -> Self {
        Self {
            code: Code::Convert(conv_type),
            line: None,
            column: None,
        }
    }
    pub(crate) const fn iter() -> Self {
        Self {
            code: Code::Iter,
            line: None,
            column: None,
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EdnError {{ code: {:?}, line: {:?}, column: {:?} }}",
            self.code, self.line, self.column
        )
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.code {
            Code::Message(m) => write!(f, "{}", m.as_ref()),
            Code::TryFromInt(e) => write!(f, "{e}"),
            _ => todo!(),
        }
    }
}

impl From<num::ParseIntError> for Code {
    fn from(e: num::ParseIntError) -> Self {
        Self::ParseNumber(ParseNumber::ParseIntError(e))
    }
}

impl From<num::ParseFloatError> for Code {
    fn from(e: num::ParseFloatError) -> Self {
        Self::ParseNumber(ParseNumber::ParseFloatError(e))
    }
}

impl From<convert::Infallible> for Error {
    fn from(_: convert::Infallible) -> Self {
        Self {
            code: Code::Infallable(),
            line: None,
            column: None,
        }
    }
}

impl From<num::TryFromIntError> for Error {
    fn from(e: num::TryFromIntError) -> Self {
        Self {
            code: Code::TryFromInt(e),
            line: None,
            column: None,
        }
    }
}
