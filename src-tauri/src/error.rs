use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum ConversionError {
    UnsupportedConversion { source: String, target: String },
    Io(std::io::Error),
    ParseError(String),
    SerializeError(String),
    InvalidFormat(String),
    ZipError(zip::result::ZipError),
    Utf8Error(String),
    FontError(String),
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedConversion { source, target } => {
                write!(f, "unsupported conversion: {} -> {}", source, target)
            }
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::ParseError(msg) => write!(f, "parse error: {}", msg),
            Self::SerializeError(msg) => write!(f, "serialize error: {}", msg),
            Self::InvalidFormat(msg) => write!(f, "invalid format: {}", msg),
            Self::ZipError(e) => write!(f, "zip error: {}", e),
            Self::Utf8Error(msg) => write!(f, "UTF-8 error: {}", msg),
            Self::FontError(msg) => write!(f, "font error: {}", msg),
        }
    }
}

impl std::error::Error for ConversionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::ZipError(e) => Some(e),
            _ => None,
        }
    }
}

// Allow ? operator for IO errors
impl From<std::io::Error> for ConversionError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<zip::result::ZipError> for ConversionError {
    fn from(e: zip::result::ZipError) -> Self {
        Self::ZipError(e)
    }
}

impl From<std::str::Utf8Error> for ConversionError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8Error(e.to_string())
    }
}

impl From<std::string::FromUtf8Error> for ConversionError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::Utf8Error(e.to_string())
    }
}

impl Serialize for ConversionError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
