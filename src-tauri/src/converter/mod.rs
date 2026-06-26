pub mod config;
pub mod data;
pub mod document;
pub mod traits;

pub use traits::{ConversionResult, Format};

use crate::error::ConversionError;

pub fn convert_file(input: &[u8], source: &Format, target: &Format) -> ConversionResult<Vec<u8>> {
    match (source.category(), target.category()) {
        ("config", "config") => {
            let input_str = std::str::from_utf8(input)?;
            config::convert_config(input_str, source, target)
                .map(|s| s.into_bytes())
        }
        ("data", "data") => {
            let input_str = std::str::from_utf8(input)?;
            data::convert_data(input_str, source, target)
                .map(|s| s.into_bytes())
        }
        ("document", "document") => document::convert_document(input, source, target),
        _ => Err(ConversionError::UnsupportedConversion {
            source: source.to_string(),
            target: target.to_string(),
        }),
    }
}
