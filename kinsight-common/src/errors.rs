use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ProcParserError {
    FileDoesntExistError(String),
    FileReadError(String),
    GPUError(String),
    ThreadSpawningError(String),
    SysCallError(String),
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
            Self::FileDoesntExistError(path) => write!(f, "FileDoesntExistError::: {}", path),
            Self::FileReadError(path) => write!(f, "FileReadError::: {}", path),
            Self::GPUError(gpu_error) => write!(f, "GPUError::: {}", gpu_error),
            Self::SysCallError(error_details) => write!(f, "SysCallError::: {}", error_details),
            Self::ThreadSpawningError(thread_info) => {
                write!(f, "ThreadSpawningError::: {}", thread_info)
            }
            Self::ParseError(err) => write!(f, "UnknownError::: {}", err),
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
