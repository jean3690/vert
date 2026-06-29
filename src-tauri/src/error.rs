use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum ConversionError {
    #[error("unsupported conversion: {from} -> {to}")]
    UnsupportedConversion { from: String, to: String },
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse error: {0}")]
    ParseError(String),
    #[error("serialize error: {0}")]
    SerializeError(String),
    #[error("invalid format: {0}")]
    InvalidFormat(String),
    #[error("zip error: {0}")]
    ZipError(#[from] zip::result::ZipError),
    #[error("UTF-8 error: {0}")]
    Utf8Error(String),
    #[error("font error: {0}")]
    FontError(String),
    #[error("output file already exists: {0}")]
    OutputExists(String),
}

// Utf8Error can come from both std::str::Utf8Error and std::string::FromUtf8Error,
// so these From impls are separate from the derive.
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
