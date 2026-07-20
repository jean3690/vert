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
    Ini,
    // Data formats
    Csv,
    // Document formats
    Epub,
    Docx,
    Pdf,
    Markdown,
    Html,
    Txt,
    // Vector / Image formats
    Svg,
    // Image formats
    Png,
    Jpeg,
    Gif,
    Webp,
    Bmp,
    Tiff,
}

impl Format {
    pub fn from_extension(path: &str) -> Option<Format> {
        let ext = path.rsplit('.').next()?.to_lowercase();
        match ext.as_str() {
            "properties" => Some(Format::Properties),
            "yaml" | "yml" => Some(Format::Yaml),
            "json" => Some(Format::Json),
            "toml" => Some(Format::Toml),
            "ini" | "cfg" | "conf" => Some(Format::Ini),
            "xml" => Some(Format::Xml),
            "csv" => Some(Format::Csv),
            "epub" => Some(Format::Epub),
            "docx" => Some(Format::Docx),
            "pdf" => Some(Format::Pdf),
            "md" | "markdown" => Some(Format::Markdown),
            "html" | "htm" => Some(Format::Html),
            "txt" | "text" => Some(Format::Txt),
            "svg" => Some(Format::Svg),
            "png" => Some(Format::Png),
            "jpg" | "jpeg" | "jpe" => Some(Format::Jpeg),
            "gif" => Some(Format::Gif),
            "webp" => Some(Format::Webp),
            "bmp" | "dib" => Some(Format::Bmp),
            "tiff" | "tif" => Some(Format::Tiff),
            _ => None,
        }
    }

    pub fn from_str(s: &str) -> ConversionResult<Format> {
        match s.to_lowercase().as_str() {
            "properties" => Ok(Format::Properties),
            "yaml" | "yml" => Ok(Format::Yaml),
            "json" => Ok(Format::Json),
            "toml" => Ok(Format::Toml),
            "ini" | "cfg" | "conf" => Ok(Format::Ini),
            "xml" => Ok(Format::Xml),
            "csv" => Ok(Format::Csv),
            "epub" => Ok(Format::Epub),
            "docx" => Ok(Format::Docx),
            "pdf" => Ok(Format::Pdf),
            "markdown" | "md" => Ok(Format::Markdown),
            "html" | "htm" => Ok(Format::Html),
            "txt" | "text" => Ok(Format::Txt),
            "svg" => Ok(Format::Svg),
            "png" => Ok(Format::Png),
            "jpg" | "jpeg" | "jpe" => Ok(Format::Jpeg),
            "gif" => Ok(Format::Gif),
            "webp" => Ok(Format::Webp),
            "bmp" | "dib" => Ok(Format::Bmp),
            "tiff" | "tif" => Ok(Format::Tiff),
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
            Format::Ini => "ini",
            Format::Csv => "csv",
            Format::Epub => "epub",
            Format::Docx => "docx",
            Format::Pdf => "pdf",
            Format::Markdown => "md",
            Format::Html => "html",
            Format::Txt => "txt",
            Format::Svg => "svg",
            Format::Png => "png",
            Format::Jpeg => "jpg",
            Format::Gif => "gif",
            Format::Webp => "webp",
            Format::Bmp => "bmp",
            Format::Tiff => "tiff",
        }
    }

    pub fn valid_targets(&self) -> Vec<Format> {
        match self {
            Format::Properties => vec![Format::Yaml, Format::Json, Format::Toml, Format::Xml, Format::Ini],
            Format::Yaml => vec![
                Format::Properties,
                Format::Json,
                Format::Toml,
                Format::Xml,
                Format::Ini,
            ],
            Format::Json => vec![
                Format::Properties,
                Format::Yaml,
                Format::Toml,
                Format::Xml,
                Format::Ini,
                Format::Csv,
            ],
            Format::Toml => vec![
                Format::Properties,
                Format::Yaml,
                Format::Json,
                Format::Xml,
                Format::Ini,
            ],
            Format::Xml => vec![
                Format::Properties,
                Format::Yaml,
                Format::Json,
                Format::Toml,
                Format::Ini,
                Format::Csv,
            ],
            Format::Ini => vec![
                Format::Properties,
                Format::Yaml,
                Format::Json,
                Format::Toml,
                Format::Xml,
            ],
            Format::Csv => vec![Format::Json, Format::Xml],
            Format::Epub => vec![Format::Pdf, Format::Html, Format::Markdown, Format::Txt],
            Format::Docx => vec![Format::Pdf, Format::Html, Format::Markdown, Format::Txt],
            Format::Pdf => vec![Format::Txt],
            Format::Markdown => vec![Format::Html, Format::Pdf, Format::Txt],
            Format::Html => vec![Format::Pdf, Format::Markdown, Format::Txt],
            Format::Txt => vec![Format::Markdown, Format::Html, Format::Pdf],

            // SVG: rasterize to any image format, extract text to TXT
            Format::Svg => vec![Format::Png, Format::Jpeg, Format::Gif, Format::Webp, Format::Bmp, Format::Tiff, Format::Txt],
            // Image formats: each can convert to any other image format
            Format::Png => vec![Format::Jpeg, Format::Gif, Format::Webp, Format::Bmp, Format::Tiff],
            Format::Jpeg => vec![Format::Png, Format::Gif, Format::Webp, Format::Bmp, Format::Tiff],
            Format::Gif => vec![Format::Png, Format::Jpeg, Format::Webp, Format::Bmp, Format::Tiff],
            Format::Webp => vec![Format::Png, Format::Jpeg, Format::Gif, Format::Bmp, Format::Tiff],
            Format::Bmp => vec![Format::Png, Format::Jpeg, Format::Gif, Format::Webp, Format::Tiff],
            Format::Tiff => vec![Format::Png, Format::Jpeg, Format::Gif, Format::Webp, Format::Bmp],
        }
    }

    pub fn category(&self) -> &str {
        match self {
            Format::Properties | Format::Yaml | Format::Json | Format::Toml | Format::Xml | Format::Ini => {
                "config"
            }
            Format::Csv => "data",
            Format::Epub | Format::Docx | Format::Pdf | Format::Markdown | Format::Html | Format::Txt => "document",
            Format::Svg | Format::Png | Format::Jpeg | Format::Gif | Format::Webp | Format::Bmp | Format::Tiff => { "image" }
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
            Format::Ini => "ini",
            Format::Csv => "csv",
            Format::Epub => "epub",
            Format::Docx => "docx",
            Format::Pdf => "pdf",
            Format::Markdown => "markdown",
            Format::Html => "html",
            Format::Txt => "txt",
            Format::Svg => "svg",
            Format::Png => "png",
            Format::Jpeg => "jpeg",
            Format::Gif => "gif",
            Format::Webp => "webp",
            Format::Bmp => "bmp",
            Format::Tiff => "tiff",
        };
        write!(f, "{}", s)
    }
}
