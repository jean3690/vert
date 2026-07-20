pub mod config;
pub mod data;
pub mod document;
pub mod image;
pub mod svg;
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
        ("image", "image") => {
            if *source == Format::Svg || *target == Format::Svg {
                svg::convert_svg(input, source, target)
            } else {
                image::convert_image(input, source, target)
            }
        }
        // SVG → TXT cross-category
        ("image", "document") => {
            if *source == Format::Svg && *target == Format::Txt {
                svg::convert_svg(input, source, target)
            } else {
                Err(ConversionError::UnsupportedConversion {
                    from: source.to_string(),
                    to: target.to_string(),
                })
            }
        }
        // Cross-category: JSON/XML ↔ CSV (both handled by the data module)
        ("config", "data") | ("data", "config") => {
            let input_str = std::str::from_utf8(input)?;
            data::convert_data(input_str, source, target)
                .map(|s| s.into_bytes())
        }
        _ => Err(ConversionError::UnsupportedConversion {
            from: source.to_string(),
            to: target.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Same-category ──

    #[test]
    fn test_json_to_yaml() {
        let output = convert_file(b"{\"name\":\"test\"}", &Format::Json, &Format::Yaml).unwrap();
        let s = std::str::from_utf8(&output).unwrap();
        assert!(s.contains("name: test"));
    }

    #[test]
    fn test_csv_to_json() {
        let output = convert_file(b"name,age\nAlice,30\n", &Format::Csv, &Format::Json).unwrap();
        let s = std::str::from_utf8(&output).unwrap();
        assert!(s.contains("Alice"));
    }

    // ── Cross-category: config ↔ data ──

    #[test]
    fn test_json_to_csv_cross_category() {
        let json = br#"[{"name":"Alice","age":"30"},{"name":"Bob","age":"25"}]"#;
        let output = convert_file(json, &Format::Json, &Format::Csv).unwrap();
        let s = std::str::from_utf8(&output).unwrap();
        assert!(s.contains("Alice"));
        assert!(s.contains("Bob"));
    }

    #[test]
    fn test_xml_to_csv_cross_category() {
        let xml = b"<root><record><name>Alice</name></record></root>";
        let output = convert_file(xml, &Format::Xml, &Format::Csv).unwrap();
        let s = std::str::from_utf8(&output).unwrap();
        assert!(s.contains("Alice"));
    }

    #[test]
    fn test_csv_to_xml_cross_category() {
        let csv = b"name,age\nAlice,30\n";
        let output = convert_file(csv, &Format::Csv, &Format::Xml).unwrap();
        let s = std::str::from_utf8(&output).unwrap();
        assert!(s.contains("Alice"));
    }

    // ── Unsupported ──

    #[test]
    fn test_pdf_to_anything_unsupported() {
        let result = convert_file(b"", &Format::Pdf, &Format::Html);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_to_document_unsupported() {
        let result = convert_file(b"{}", &Format::Json, &Format::Pdf);
        assert!(result.is_err());
    }

    // ── Document ──

    #[test]
    fn test_markdown_to_html() {
        let output = convert_file(b"# Hello", &Format::Markdown, &Format::Html).unwrap();
        let s = std::str::from_utf8(&output).unwrap();
        assert!(s.contains("<h1>Hello</h1>"));
    }

    // ── Image ──

    #[test]
    fn test_image_png_to_jpeg() {
        let buf: ::image::ImageBuffer<::image::Rgba<u8>, Vec<u8>> =
            ::image::ImageBuffer::from_raw(1, 1, vec![255, 0, 0, 255]).unwrap();
        let mut cursor = std::io::Cursor::new(Vec::new());
        ::image::DynamicImage::from(buf)
            .write_to(&mut cursor, ::image::ImageFormat::Png)
            .unwrap();
        let png = cursor.into_inner();
        let output = convert_file(&png, &Format::Png, &Format::Jpeg).unwrap();
        assert_eq!(output[..2], [0xFF, 0xD8]);
    }

    #[test]
    fn test_image_jpeg_to_png() {
        let buf: ::image::ImageBuffer<::image::Rgb<u8>, Vec<u8>> =
            ::image::ImageBuffer::from_raw(1, 1, vec![255, 0, 0]).unwrap();
        let mut cursor = std::io::Cursor::new(Vec::new());
        ::image::DynamicImage::from(buf)
            .write_to(&mut cursor, ::image::ImageFormat::Jpeg)
            .unwrap();
        let jpeg = cursor.into_inner();
        let output = convert_file(&jpeg, &Format::Jpeg, &Format::Png).unwrap();
        assert_eq!(&output[..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }
}
