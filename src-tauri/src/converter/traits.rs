use crate::error::ConversionError;
use std::fmt;

pub type ConversionResult<T> = Result<T, ConversionError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    // Config formats
    Properties,
    Yaml,
    Json,
    Toml,
    Xml,
    // Data formats
    Csv,
    // Document formats
    Docx,
    Pdf,
    Markdown,
    Html,
}

impl Format {
    pub fn from_extension(path: &str) -> Option<Format> {
        let ext = path.rsplit('.').next()?.to_lowercase();
        match ext.as_str() {
            "properties" => Some(Format::Properties),
            "yaml" | "yml" => Some(Format::Yaml),
            "json" => Some(Format::Json),
            "toml" => Some(Format::Toml),
            "xml" => Some(Format::Xml),
            "csv" => Some(Format::Csv),
            "docx" => Some(Format::Docx),
            "pdf" => Some(Format::Pdf),
            "md" | "markdown" => Some(Format::Markdown),
            "html" | "htm" => Some(Format::Html),
            _ => None,
        }
    }

    pub fn from_str(s: &str) -> ConversionResult<Format> {
        match s.to_lowercase().as_str() {
            "properties" => Ok(Format::Properties),
            "yaml" | "yml" => Ok(Format::Yaml),
            "json" => Ok(Format::Json),
            "toml" => Ok(Format::Toml),
            "xml" => Ok(Format::Xml),
            "csv" => Ok(Format::Csv),
            "docx" => Ok(Format::Docx),
            "pdf" => Ok(Format::Pdf),
            "markdown" | "md" => Ok(Format::Markdown),
            "html" | "htm" => Ok(Format::Html),
            _ => Err(ConversionError::InvalidFormat(format!(
                "unknown format: {}",
                s
            ))),
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            Format::Properties => "properties",
            Format::Yaml => "yaml",
            Format::Json => "json",
            Format::Toml => "toml",
            Format::Xml => "xml",
            Format::Csv => "csv",
            Format::Docx => "docx",
            Format::Pdf => "pdf",
            Format::Markdown => "md",
            Format::Html => "html",
        }
    }

    pub fn valid_targets(&self) -> Vec<Format> {
        match self {
            Format::Properties => vec![Format::Yaml, Format::Json, Format::Toml, Format::Xml],
            Format::Yaml => vec![
                Format::Properties,
                Format::Json,
                Format::Toml,
                Format::Xml,
            ],
            Format::Json => vec![
                Format::Properties,
                Format::Yaml,
                Format::Toml,
                Format::Xml,
                Format::Csv,
            ],
            Format::Toml => vec![
                Format::Properties,
                Format::Yaml,
                Format::Json,
                Format::Xml,
            ],
            Format::Xml => vec![
                Format::Properties,
                Format::Yaml,
                Format::Json,
                Format::Toml,
                Format::Csv,
            ],
            Format::Csv => vec![Format::Json, Format::Xml],
            Format::Docx => vec![Format::Pdf, Format::Html, Format::Markdown],
            Format::Pdf => vec![],
            Format::Markdown => vec![Format::Html, Format::Pdf],
            Format::Html => vec![Format::Pdf, Format::Markdown],
        }
    }

    pub fn category(&self) -> &str {
        match self {
            Format::Properties | Format::Yaml | Format::Json | Format::Toml | Format::Xml => {
                "config"
            }
            Format::Csv => "data",
            Format::Docx | Format::Pdf | Format::Markdown | Format::Html => "document",
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Format::Properties => "properties",
            Format::Yaml => "yaml",
            Format::Json => "json",
            Format::Toml => "toml",
            Format::Xml => "xml",
            Format::Csv => "csv",
            Format::Docx => "docx",
            Format::Pdf => "pdf",
            Format::Markdown => "markdown",
            Format::Html => "html",
        };
        write!(f, "{}", s)
    }
}
