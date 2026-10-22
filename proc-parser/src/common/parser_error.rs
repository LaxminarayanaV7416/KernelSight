use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ProcParserError {
    FileReadError(String),
    GPUError,
    ParseError(Box<dyn Error + Send + Sync + 'static>),
}

impl ProcParserError {
    /// Wrap an error from any crate as an unknown parser error.
    pub fn unknown<E>(err: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self::ParseError(Box::new(err))
    }
}

impl Display for ProcParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileReadError(path) => write!(f, "Something is wrong {}", path),
            Self::GPUError => write!(f, "GPU error"),
            Self::ParseError(err) => write!(f, "{err}"),
        }
    }
}

impl Error for ProcParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParseError(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

/// Allows `?` to convert standard I/O errors automatically.
// impl From<std::io::Error> for ProcParserError {
//     fn from(err: std::io::Error) -> Self {
//         Self::unknown(err)
//     }
// }

/// Allows an already boxed error to be converted automatically.
impl From<Box<dyn Error + Send + Sync + 'static>> for ProcParserError {
    fn from(err: Box<dyn Error + Send + Sync + 'static>) -> Self {
        Self::ParseError(err)
    }
}

pub type Result<T> = std::result::Result<T, ProcParserError>;
