use crate::converter::traits::{ConversionResult, Format};
use crate::error::ConversionError;
use resvg::tiny_skia;
use resvg::usvg;

/// Convert SVG content to another format.
///
/// SVG can produce:
/// - Raster image formats (PNG, JPEG, GIF, WebP, BMP, TIFF) via resvg
/// - Plain text (TXT) via XML text extraction
pub fn convert_svg(input: &[u8], source: &Format, target: &Format) -> ConversionResult<Vec<u8>> {
    let svg_str =
        std::str::from_utf8(input).map_err(|e| ConversionError::Utf8Error(e.to_string()))?;

    match target {
        Format::Txt => svg_to_txt(svg_str).map(|s| s.into_bytes()),
        Format::Png
        | Format::Jpeg
        | Format::Gif
        | Format::Webp
        | Format::Bmp
        | Format::Tiff => svg_to_image(svg_str, target),
        _ => Err(ConversionError::UnsupportedConversion {
            from: source.to_string(),
            to: target.to_string(),
        }),
    }
}

/// Convert a TXT string to a basic SVG wrapper.
pub fn txt_to_svg(input: &str) -> String {
    let escaped = input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="800" height="600">
  <rect width="100%" height="100%" fill="white"/>
  <text x="20" y="40" font-family="monospace" font-size="14" fill="black">{}</text>
</svg>"#,
        escaped
    )
}

// ── SVG → TXT (extract all text nodes) ──

fn svg_to_txt(svg: &str) -> ConversionResult<String> {
    use quick_xml::events::Event;
    use quick_xml::Reader;

    let mut reader = Reader::from_str(svg);
    reader.config_mut().trim_text(true);

    let mut text = String::new();
    let mut buf = Vec::new();
    let mut in_text = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = e.local_name().as_ref().to_vec();
                in_text = tag == b"text" || tag == b"tspan" || tag == b"desc";
            }
            Ok(Event::Text(ref e)) => {
                if in_text {
                    if let Ok(t) = e.unescape() {
                        text.push_str(&t);
                        text.push(' ');
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = e.local_name().as_ref().to_vec();
                if tag == b"text" || tag == b"tspan" {
                    in_text = false;
                    text.push('\n');
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(ConversionError::ParseError(format!("svg xml: {}", e)));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(text.trim().to_string())
}

// ── SVG → raster image (via resvg) ──

fn svg_to_image(svg: &str, target: &Format) -> ConversionResult<Vec<u8>> {
    let tree = usvg::Tree::from_str(svg, &usvg::Options::default())
        .map_err(|e| ConversionError::ParseError(format!("svg parse: {}", e)))?;

    let width = tree.size().width() as u32;
    let height = tree.size().height() as u32;

    if width == 0 || height == 0 {
        return Err(ConversionError::ParseError(
            "SVG has zero dimensions".into(),
        ));
    }

    // Clamp to reasonable maximum (4096px)
    let (width, height) = if width > 4096 || height > 4096 {
        let ratio = (width as f64) / (height as f64);
        if width > height {
            (4096, (4096_f64 / ratio).round() as u32)
        } else {
            ((4096_f64 * ratio).round() as u32, 4096)
        }
    } else {
        (width, height)
    };

    // Create pixel buffer and render
    let mut pixmap = tiny_skia::Pixmap::new(width, height).ok_or_else(|| {
        ConversionError::SerializeError("failed to create pixel buffer".into())
    })?;

    let scale_x = width as f32 / tree.size().width();
    let scale_y = height as f32 / tree.size().height();
    let ts = tiny_skia::Transform::from_scale(scale_x, scale_y);

    let mut pm = pixmap.as_mut();
    resvg::render(&tree, ts, &mut pm);

    // Convert to image::DynamicImage and encode
    let img: image::DynamicImage = image::DynamicImage::ImageRgba8(
        image::ImageBuffer::from_raw(width, height, pixmap.data().to_vec()).ok_or_else(|| {
            ConversionError::SerializeError("failed to create image".into())
        })?,
    );

    let mut output = Vec::new();
    match target {
        Format::Png => {
            let mut c = std::io::Cursor::new(&mut output);
            img.write_to(&mut c, image::ImageFormat::Png)
                .map_err(|e| ConversionError::SerializeError(format!("png: {}", e)))?;
        }
        Format::Jpeg => {
            let rgb = img.to_rgb8();
            let mut enc = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, 90);
            enc.encode(rgb.as_raw(), rgb.width(), rgb.height(), image::ExtendedColorType::Rgb8)
                .map_err(|e| ConversionError::SerializeError(format!("jpeg: {}", e)))?;
        }
        Format::Gif => {
            let mut c = std::io::Cursor::new(&mut output);
            img.write_to(&mut c, image::ImageFormat::Gif)
                .map_err(|e| ConversionError::SerializeError(format!("gif: {}", e)))?;
        }
        Format::Webp => {
            let mut c = std::io::Cursor::new(&mut output);
            img.write_to(&mut c, image::ImageFormat::WebP)
                .map_err(|e| ConversionError::SerializeError(format!("webp: {}", e)))?;
        }
        Format::Bmp => {
            let mut c = std::io::Cursor::new(&mut output);
            img.write_to(&mut c, image::ImageFormat::Bmp)
                .map_err(|e| ConversionError::SerializeError(format!("bmp: {}", e)))?;
        }
        Format::Tiff => {
            let mut c = std::io::Cursor::new(&mut output);
            img.write_to(&mut c, image::ImageFormat::Tiff)
                .map_err(|e| ConversionError::SerializeError(format!("tiff: {}", e)))?;
        }
        _ => unreachable!(),
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINI_SVG: &str = r#"<?xml version="1.0"?>
<svg xmlns="http://www.w3.org/2000/svg" width="100" height="50">
  <rect width="100%" height="100%" fill="red"/>
  <text x="10" y="30" font-size="20" fill="white">Hello</text>
</svg>"#;

    #[test]
    fn test_svg_to_png() {
        let output = convert_svg(MINI_SVG.as_bytes(), &Format::Svg, &Format::Png).unwrap();
        assert!(!output.is_empty());
        assert_eq!(&output[..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_svg_to_jpeg() {
        let output = convert_svg(MINI_SVG.as_bytes(), &Format::Svg, &Format::Jpeg).unwrap();
        assert!(!output.is_empty());
        assert_eq!(output[..2], [0xFF, 0xD8]);
    }

    #[test]
    fn test_svg_to_txt() {
        let output = convert_svg(MINI_SVG.as_bytes(), &Format::Svg, &Format::Txt).unwrap();
        let s = std::str::from_utf8(&output).unwrap();
        assert!(s.contains("Hello"));
    }

    #[test]
    fn test_txt_to_svg() {
        let svg = txt_to_svg("Hello World");
        assert!(svg.starts_with("<?xml"));
        assert!(svg.contains("Hello World"));
        let output = convert_svg(svg.as_bytes(), &Format::Svg, &Format::Txt).unwrap();
        let s = std::str::from_utf8(&output).unwrap();
        assert!(s.contains("Hello World"));
    }

    #[test]
    fn test_svg_to_webp() {
        let output = convert_svg(MINI_SVG.as_bytes(), &Format::Svg, &Format::Webp).unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_invalid_svg() {
        let result = convert_svg(b"not svg", &Format::Svg, &Format::Png);
        assert!(result.is_err());
    }
}
