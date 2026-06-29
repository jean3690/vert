use crate::converter;
use crate::error::ConversionError;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct ConvertResult {
    #[serde(rename = "outputPath")]
    pub output_path: String,
    #[serde(rename = "outputSize")]
    pub output_size: u64,
}

#[tauri::command]
pub fn convert_file(
    file_path: String,
    source_format: String,
    target_format: String,
) -> Result<ConvertResult, ConversionError> {
    let input = std::fs::read(&file_path)?;

    let source = converter::Format::from_str(&source_format)?;
    let target = converter::Format::from_str(&target_format)?;

    let source_path = Path::new(&file_path);
    let output_path = source_path.with_extension(target.extension());

    if output_path.exists() {
        return Err(ConversionError::OutputExists(
            output_path.to_string_lossy().to_string(),
        ));
    }

    let output = converter::convert_file(&input, &source, &target)?;
    std::fs::write(&output_path, &output)?;

    let size = std::fs::metadata(&output_path)
        .map(|m| m.len())
        .unwrap_or(output.len() as u64);

    Ok(ConvertResult {
        output_path: output_path.to_string_lossy().to_string(),
        output_size: size,
    })
}

#[tauri::command]
pub fn get_valid_targets(source_format: String) -> Result<Vec<String>, ConversionError> {
    let source = converter::Format::from_str(&source_format)?;
    Ok(source.valid_targets().iter().map(|f| f.to_string()).collect())
}
