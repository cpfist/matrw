use core::fmt;
use std::fmt::Display;

/// Error types
#[derive(Debug)]
pub enum MatrwError {
    IoError(std::io::Error),
    BinrwError(binrw::Error),
    MatFile73Error,
    AccessError(String),
    SerdeError(String),
    TypeConstruction(String),
}

impl fmt::Display for MatrwError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatrwError::IoError(e) => write!(f, "IO error {}", e),
            MatrwError::BinrwError(e) => write!(f, "binrw error {}", e),
            MatrwError::MatFile73Error => write!(f, "MAT-file Version 7.3 not yet supported!"),
            MatrwError::AccessError(msg) => write!(f, "{}", msg),
            MatrwError::SerdeError(e) => write!(f, "Serde error {}", e),
            MatrwError::TypeConstruction(msg) => write!(f, "Type construction error {}", msg),
        }
    }
}

impl std::error::Error for MatrwError {}

impl From<binrw::Error> for MatrwError {
    fn from(value: binrw::Error) -> Self {
        MatrwError::BinrwError(value)
    }
}

impl From<std::io::Error> for MatrwError {
    fn from(value: std::io::Error) -> Self {
        MatrwError::IoError(value)
    }
}

impl serde::ser::Error for MatrwError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::SerdeError(msg.to_string())
    }
}

impl serde::de::Error for MatrwError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::SerdeError(msg.to_string())
    }
}
