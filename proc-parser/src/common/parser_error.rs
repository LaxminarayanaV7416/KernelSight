use std::error::Error;

#[derive(Debug)]
pub enum ProcParserError {
    ProcFileReadError,
    ProcPIDFileReadError,
    CgroupFileReadError,
    GPUError,
    UnKnownError(Box<dyn Error>),
}

pub type Result<T> = std::result::Result<T, ProcParserError>;
