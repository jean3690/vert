use crate::converter::traits::{ConversionResult, Format};
use crate::error::ConversionError;

/// Convert an image from one format to another.
///
/// All six image formats (PNG, JPEG, GIF, WebP, BMP, TIFF) form a fully-
/// connected clique — any input can produce any output. The `image` crate
/// decodes to a `DynamicImage` then re-encodes in the target format.
pub fn convert_image(input: &[u8], source: &Format, target: &Format) -> ConversionResult<Vec<u8>> {
    let img = load_image(input, source)?;
    save_image(&img, target)
}

fn image_format_from_format(f: &Format) -> ConversionResult<image::ImageFormat> {
    match f {
        Format::Png => Ok(image::ImageFormat::Png),
        Format::Jpeg => Ok(image::ImageFormat::Jpeg),
        Format::Gif => Ok(image::ImageFormat::Gif),
        Format::Webp => Ok(image::ImageFormat::WebP),
        Format::Bmp => Ok(image::ImageFormat::Bmp),
        Format::Tiff => Ok(image::ImageFormat::Tiff),
        _ => Err(ConversionError::UnsupportedConversion {
            from: f.to_string(),
            to: "(image)".into(),
        }),
    }
}

fn load_image(input: &[u8], format: &Format) -> ConversionResult<image::DynamicImage> {
    let img_format = image_format_from_format(format)?;
    let reader = image::ImageReader::with_format(std::io::Cursor::new(input), img_format)
        .with_guessed_format()
        .map_err(|e| ConversionError::ParseError(format!("image format detection: {}", e)))?;

    reader
        .decode()
        .map_err(|e| ConversionError::ParseError(format!("image decode: {}", e)))
}

fn save_image(img: &image::DynamicImage, format: &Format) -> ConversionResult<Vec<u8>> {
    let img_format = image_format_from_format(format)?;

    // JPEG doesn't support alpha — convert RGBA to RGB for best quality.
    let img = if matches!(format, Format::Jpeg) && img.color().has_alpha() {
        img.to_rgb8().into()
    } else {
        img.clone()
    };

    let mut cursor = std::io::Cursor::new(Vec::new());
    img.write_to(&mut cursor, img_format)
        .map_err(|e| ConversionError::SerializeError(format!("image encode: {}", e)))?;

    Ok(cursor.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a minimal 1×1 RGBA PNG in memory by using the `image` crate.
    fn make_png() -> Vec<u8> {
        use image::ImageBuffer;
        // ImageBuffer::from_raw takes (width, height, pixel_data)
        // For RGBA the subpixel type is u8
        let buf: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(1, 1, vec![255, 0, 0, 255]).unwrap();
        let mut cursor = std::io::Cursor::new(Vec::new());
        image::DynamicImage::from(buf)
            .write_to(&mut cursor, image::ImageFormat::Png)
            .unwrap();
        cursor.into_inner()
    }

    /// Create a minimal 1×1 RGB JPEG in memory.
    fn make_jpeg() -> Vec<u8> {
        use image::ImageBuffer;
        let buf: ImageBuffer<image::Rgb<u8>, Vec<u8>> =
            ImageBuffer::from_raw(1, 1, vec![255, 0, 0]).unwrap();
        let mut cursor = std::io::Cursor::new(Vec::new());
        image::DynamicImage::from(buf)
            .write_to(&mut cursor, image::ImageFormat::Jpeg)
            .unwrap();
        cursor.into_inner()
    }

    #[test]
    fn test_png_to_jpeg() {
        let output = convert_image(&make_png(), &Format::Png, &Format::Jpeg).unwrap();
        assert!(!output.is_empty());
        assert_eq!(output[..2], [0xFF, 0xD8]);
    }

    #[test]
    fn test_png_to_webp() {
        let output = convert_image(&make_png(), &Format::Png, &Format::Webp).unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_png_to_gif() {
        let output = convert_image(&make_png(), &Format::Png, &Format::Gif).unwrap();
        assert!(!output.is_empty());
        assert_eq!(&output[..3], b"GIF");
    }

    #[test]
    fn test_png_to_bmp() {
        let output = convert_image(&make_png(), &Format::Png, &Format::Bmp).unwrap();
        assert!(!output.is_empty());
        assert_eq!(output[..2], [0x42, 0x4D]);
    }

    #[test]
    fn test_png_to_tiff() {
        let output = convert_image(&make_png(), &Format::Png, &Format::Tiff).unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_jpeg_to_png() {
        let output = convert_image(&make_jpeg(), &Format::Jpeg, &Format::Png).unwrap();
        assert!(!output.is_empty());
        assert_eq!(&output[..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_all_image_roundtrip_png() {
        let targets = [
            Format::Jpeg,
            Format::Gif,
            Format::Webp,
            Format::Bmp,
            Format::Tiff,
        ];
        for target in &targets {
            let result = convert_image(&make_png(), &Format::Png, target);
            assert!(
                result.is_ok(),
                "PNG -> {} failed: {:?}",
                target,
                result.err()
            );
            assert!(!result.unwrap().is_empty());
        }
    }

    #[test]
    fn test_invalid_image_returns_error() {
        let result = convert_image(b"not an image", &Format::Png, &Format::Jpeg);
        assert!(result.is_err());
    }
}
