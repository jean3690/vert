use crate::converter::traits::{ConversionResult, Format};
use crate::error::ConversionError;
use genpdf::Element as _;
use std::io::Cursor;

// ═══════════════════════════════════════════════════════════════
// Shared: DOCX element extraction
// ═══════════════════════════════════════════════════════════════

#[derive(Debug)]
struct DocElement {
    kind: ElementKind,
    text: String,
    bold: bool,
    italic: bool,
    font_size: f32,
}

#[derive(Debug, PartialEq)]
enum ElementKind {
    Normal,
    Heading1,
    Heading2,
    Heading3,
    ListItem,
}

fn extract_docx_elements(data: &[u8]) -> ConversionResult<Vec<DocElement>> {
    let reader = Cursor::new(data);
    let mut archive = zip::ZipArchive::new(reader).map_err(ConversionError::ZipError)?;

    let doc_entry = archive
        .by_name("word/document.xml")
        .map_err(|_| ConversionError::ParseError("DOCX missing word/document.xml".into()))?;

    let mut xml_data = Vec::new();
    let mut doc_reader = doc_entry;
    std::io::copy(&mut doc_reader, &mut xml_data)
        .map_err(|e| ConversionError::ParseError(format!("failed to read document.xml: {}", e)))?;

    let doc_str =
        String::from_utf8(xml_data).map_err(|e| ConversionError::Utf8Error(e.to_string()))?;

    parse_docx_xml(&doc_str)
}

fn parse_docx_xml(xml: &str) -> ConversionResult<Vec<DocElement>> {
    use quick_xml::events::Event;
    use quick_xml::Reader;

    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut elements = Vec::new();
    let mut in_p = false;
    let mut in_r = false;
    let mut in_t = false;
    let mut current_kind = ElementKind::Normal;
    let mut current_bold = false;
    let mut current_italic = false;
    let mut current_font_size: f32 = 11.0;
    let mut current_text = String::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name_bytes = e.name().as_ref().to_vec();
                let name = String::from_utf8_lossy(&name_bytes);
                match name.as_ref() {
                    "w:p" => {
                        in_p = true;
                        current_kind = ElementKind::Normal;
                        current_text.clear();
                    }
                    "w:pPr" if in_p => {
                        if let Some(style) = read_inner_xml_to_string(&mut reader, &mut buf) {
                            if let Some(style_val) =
                                extract_attribute_value(&style, "w:pStyle", "w:val")
                            {
                                current_kind = match style_val.to_lowercase().as_str() {
                                    "heading1" | "1" => ElementKind::Heading1,
                                    "heading2" | "2" => ElementKind::Heading2,
                                    "heading3" | "3" => ElementKind::Heading3,
                                    _ => ElementKind::Normal,
                                };
                            }
                        }
                        continue;
                    }
                    "w:r" => {
                        in_r = true;
                        current_bold = false;
                        current_italic = false;
                        current_font_size = 11.0;
                    }
                    "w:rPr" if in_r => {
                        if let Some(rpr_xml) =
                            read_inner_xml_to_string(&mut reader, &mut buf)
                        {
                            current_bold =
                                rpr_xml.contains("<w:b") || rpr_xml.contains("<w:b/");
                            current_italic =
                                rpr_xml.contains("<w:i") || rpr_xml.contains("<w:i/");
                            if let Some(sz_str) =
                                extract_attribute_value(&rpr_xml, "w:sz", "w:val")
                            {
                                if let Ok(half_pts) = sz_str.parse::<f32>() {
                                    current_font_size = half_pts / 2.0;
                                }
                            }
                        }
                        continue;
                    }
                    "w:t" => in_t = true,
                    _ => {}
                }
            }
            Ok(Event::Text(ref e)) => {
                if in_t {
                    if let Ok(text) = e.unescape() {
                        current_text.push_str(&text);
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                let name_bytes = e.name().as_ref().to_vec();
                let name = String::from_utf8_lossy(&name_bytes);
                match name.as_ref() {
                    "w:t" => in_t = false,
                    "w:r" => in_r = false,
                    "w:p" => {
                        in_p = false;
                        if !current_text.trim().is_empty() {
                            elements.push(DocElement {
                                kind: std::mem::replace(&mut current_kind, ElementKind::Normal),
                                text: std::mem::take(&mut current_text),
                                bold: current_bold,
                                italic: current_italic,
                                font_size: current_font_size,
                            });
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Ok(Event::Empty(ref e)) => {
                let name_bytes = e.name().as_ref().to_vec();
                let name = String::from_utf8_lossy(&name_bytes);
                if name.as_ref() == "w:br" && in_p {
                    current_text.push('\n');
                }
            }
            Err(e) => {
                return Err(ConversionError::ParseError(format!("XML parse error: {}", e)));
            }
            _ => {}
        }
        buf.clear();
    }
    Ok(elements)
}

fn read_inner_xml_to_string<B: std::io::BufRead>(
    reader: &mut quick_xml::Reader<B>,
    buf: &mut Vec<u8>,
) -> Option<String> {
    use quick_xml::events::Event;
    let mut depth: i32 = 1;
    let mut result = String::new();
    loop {
        match reader.read_event_into(buf) {
            Ok(Event::Start(ref e)) => {
                depth += 1;
                let name_bytes = e.name().as_ref().to_vec();
                let name = String::from_utf8_lossy(&name_bytes);
                result.push_str(&format!("<{}", name));
                for attr in e.attributes().flatten() {
                    let key_bytes = attr.key.0.to_vec();
                    let key = String::from_utf8_lossy(&key_bytes);
                    let val = String::from_utf8_lossy(&attr.value);
                    result.push_str(&format!(" {}=\"{}\"", key, val));
                }
                result.push('>');
            }
            Ok(Event::Empty(ref e)) => {
                let name_bytes = e.name().as_ref().to_vec();
                let name = String::from_utf8_lossy(&name_bytes);
                result.push_str(&format!("<{}", name));
                for attr in e.attributes().flatten() {
                    let key_bytes = attr.key.0.to_vec();
                    let key = String::from_utf8_lossy(&key_bytes);
                    let val = String::from_utf8_lossy(&attr.value);
                    result.push_str(&format!(" {}=\"{}\"", key, val));
                }
                result.push_str("/>");
            }
            Ok(Event::End(_)) => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
        buf.clear();
    }
    Some(result)
}

fn extract_attribute_value<'a>(xml: &'a str, element: &str, attr: &str) -> Option<&'a str> {
    let open_tag = format!("<{} ", element);
    let start = xml.find(&open_tag)?;
    let rest = &xml[start + open_tag.len()..];
    let end = rest.find('>')?;
    let attrs_str = &rest[..end];
    let attr_eq = format!("{}=\"", attr);
    let attr_start = attrs_str.find(&attr_eq)?;
    let val_start = attr_start + attr_eq.len();
    let val_end = attrs_str[val_start..].find('"')?;
    Some(&attrs_str[val_start..val_start + val_end])
}

// ═══════════════════════════════════════════════════════════════
// DOCX → HTML / Markdown
// ═══════════════════════════════════════════════════════════════

fn elements_to_html(elements: &[DocElement]) -> String {
    let mut html = String::from("<!DOCTYPE html>\n<html>\n<body>\n");
    for elem in elements {
        let mut inner = elem.text.clone();
        if elem.bold {
            inner = format!("<strong>{}</strong>", inner);
        }
        if elem.italic {
            inner = format!("<em>{}</em>", inner);
        }
        match elem.kind {
            ElementKind::Heading1 => html.push_str(&format!("<h1>{}</h1>\n", inner)),
            ElementKind::Heading2 => html.push_str(&format!("<h2>{}</h2>\n", inner)),
            ElementKind::Heading3 => html.push_str(&format!("<h3>{}</h3>\n", inner)),
            ElementKind::ListItem => html.push_str(&format!("<li>{}</li>\n", inner)),
            ElementKind::Normal => html.push_str(&format!("<p>{}</p>\n", inner)),
        }
    }
    html.push_str("</body>\n</html>\n");
    html
}

fn elements_to_markdown(elements: &[DocElement]) -> String {
    let mut md = String::new();
    for elem in elements {
        let mut text = elem.text.clone();
        if elem.bold {
            text = format!("**{}**", text);
        }
        if elem.italic {
            text = format!("*{}*", text);
        }
        match elem.kind {
            ElementKind::Heading1 => md.push_str(&format!("# {}\n\n", text)),
            ElementKind::Heading2 => md.push_str(&format!("## {}\n\n", text)),
            ElementKind::Heading3 => md.push_str(&format!("### {}\n\n", text)),
            ElementKind::ListItem => md.push_str(&format!("- {}\n", text)),
            ElementKind::Normal => md.push_str(&format!("{}\n\n", text)),
        }
    }
    md
}

fn docx_to_html(data: &[u8]) -> ConversionResult<Vec<u8>> {
    let elements = extract_docx_elements(data)?;
    Ok(elements_to_html(&elements).into_bytes())
}

fn docx_to_markdown(data: &[u8]) -> ConversionResult<Vec<u8>> {
    let elements = extract_docx_elements(data)?;
    Ok(elements_to_markdown(&elements).into_bytes())
}

// ═══════════════════════════════════════════════════════════════
// Markdown → HTML / PDF
// ═══════════════════════════════════════════════════════════════

pub fn markdown_to_html(input: &str) -> String {
    let parser = pulldown_cmark::Parser::new(input);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    html
}

fn md_events_to_pdf(input: &str) -> ConversionResult<Vec<u8>> {
    use pulldown_cmark::{Event, Tag, TagEnd};

    let font_family = load_font_family()?;
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Converted Document");
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(20);
    doc.set_page_decorator(decorator);

    let mut current_text = String::new();
    let mut bold = false;
    let mut italic = false;
    let mut in_heading: Option<i32> = None;

    let parser = pulldown_cmark::Parser::new(input).into_iter();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                in_heading = Some(level as i32);
            }
            Event::Start(Tag::Paragraph) => {}
            Event::Start(Tag::Item) => {
                current_text.push_str("\u{2022} ");
            }
            Event::Start(Tag::Strong) => bold = true,
            Event::Start(Tag::Emphasis) => italic = true,
            Event::End(TagEnd::Heading(_)) => {
                if !current_text.trim().is_empty() {
                    if in_heading.is_some() {
                        doc.push(genpdf::elements::Break::new(1));
                    }
                    doc.push(
                        genpdf::elements::Paragraph::new(current_text.trim().to_string())
                            .styled(genpdf::style::Style::new().bold()),
                    );
                    current_text.clear();
                }
                in_heading = None;
            }
            Event::End(TagEnd::Paragraph) => {
                if !current_text.trim().is_empty() {
                    let mut style = genpdf::style::Style::new();
                    if bold { style = style.bold(); }
                    if italic { style = style.italic(); }
                    doc.push(
                        genpdf::elements::Paragraph::new(current_text.trim().to_string())
                            .padded((0, 2))
                            .styled(style),
                    );
                    current_text.clear();
                }
                bold = false;
                italic = false;
            }
            Event::End(TagEnd::Item) => {
                if !current_text.trim().is_empty() {
                    let mut style = genpdf::style::Style::new();
                    if bold { style = style.bold(); }
                    if italic { style = style.italic(); }
                    doc.push(
                        genpdf::elements::Paragraph::new(current_text.trim().to_string())
                            .styled(style),
                    );
                    current_text.clear();
                }
                bold = false;
                italic = false;
            }
            Event::End(TagEnd::Strong) => bold = false,
            Event::End(TagEnd::Emphasis) => italic = false,
            Event::Text(text) => {
                current_text.push_str(&text);
            }
            Event::SoftBreak => {
                current_text.push(' ');
            }
            Event::HardBreak => {
                current_text.push('\n');
            }
            _ => {}
        }
    }

    // Flush any remaining text
    if !current_text.trim().is_empty() {
        doc.push(
            genpdf::elements::Paragraph::new(current_text.trim().to_string())
                .padded((0, 2)),
        );
    }

    let mut output = Vec::new();
    doc.render(&mut output)
        .map_err(|e| ConversionError::SerializeError(format!("pdf render: {}", e)))?;
    Ok(output)
}

fn markdown_to_pdf(input: &str) -> ConversionResult<Vec<u8>> {
    md_events_to_pdf(input)
}

// ═══════════════════════════════════════════════════════════════
// HTML → PDF / Markdown
// ═══════════════════════════════════════════════════════════════

fn html_to_pdf(input: &str) -> ConversionResult<Vec<u8>> {
    let font_family = load_font_family()?;
    let mut gen_doc = genpdf::Document::new(font_family);
    gen_doc.set_title("Converted Document");
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(20);
    gen_doc.set_page_decorator(decorator);

    let document = scraper::Html::parse_document(input);
    let selector = scraper::Selector::parse("h1,h2,h3,h4,h5,h6,p,li,blockquote,div")
        .expect("valid CSS selector");

    for el in document.select(&selector) {
        let tag = el.value().name().to_lowercase();
        let text: String = el.text().collect();
        let trimmed = text.trim().to_string();
        if trimmed.is_empty() {
            continue;
        }
        let is_heading = matches!(tag.as_str(), "h1" | "h2" | "h3" | "h4" | "h5" | "h6");
        if is_heading {
            gen_doc.push(genpdf::elements::Break::new(1));
        }
        gen_doc.push(
            genpdf::elements::Paragraph::new(&trimmed)
                .styled(if is_heading {
                    genpdf::style::Style::new().bold()
                } else {
                    genpdf::style::Style::new()
                })
                .padded((0, 2)),
        );
    }

    let mut output = Vec::new();
    gen_doc.render(&mut output)
        .map_err(|e| ConversionError::SerializeError(format!("pdf render: {}", e)))?;
    Ok(output)
}

fn html_to_markdown(input: &str) -> String {
    let document = scraper::Html::parse_document(input);
    let selector = scraper::Selector::parse("h1,h2,h3,h4,h5,h6,p,li,blockquote,div")
        .expect("valid CSS selector");
    let mut md = String::new();

    for el in document.select(&selector) {
        let tag = el.value().name().to_lowercase();
        let text: String = el.text().collect();
        let trimmed = text.trim().to_string();
        if trimmed.is_empty() {
            continue;
        }
        match tag.as_str() {
            "h1" => md.push_str(&format!("# {}\n\n", trimmed)),
            "h2" => md.push_str(&format!("## {}\n\n", trimmed)),
            "h3" => md.push_str(&format!("### {}\n\n", trimmed)),
            "h4" | "h5" | "h6" => md.push_str(&format!("#### {}\n\n", trimmed)),
            "li" => md.push_str(&format!("- {}\n", trimmed)),
            _ => md.push_str(&format!("{}\n\n", trimmed)),
        }
    }
    md
}

// ═══════════════════════════════════════════════════════════════
// PDF rendering (shared)
// ═══════════════════════════════════════════════════════════════

fn render_docx_to_pdf(elements: &[DocElement]) -> ConversionResult<Vec<u8>> {
    let font_family = load_font_family()?;
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Converted Document");
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(20);
    doc.set_page_decorator(decorator);

    for elem in elements {
        let style = {
            let mut s = genpdf::style::Style::new();
            if elem.bold { s = s.bold(); }
            if elem.italic { s = s.italic(); }
            s
        };
        match elem.kind {
            ElementKind::Heading1 => {
                doc.push(genpdf::elements::Break::new(1));
                doc.push(
                    genpdf::elements::Paragraph::new(&elem.text)
                        .styled(genpdf::style::Style::new().bold()),
                );
            }
            ElementKind::Heading2 => {
                doc.push(genpdf::elements::Break::new(1));
                doc.push(
                    genpdf::elements::Paragraph::new(&elem.text)
                        .styled(genpdf::style::Style::new().bold()),
                );
            }
            ElementKind::Heading3 => {
                doc.push(genpdf::elements::Break::new(1));
                doc.push(
                    genpdf::elements::Paragraph::new(&elem.text)
                        .styled(genpdf::style::Style::new().bold()),
                );
            }
            ElementKind::ListItem => {
                doc.push(
                    genpdf::elements::Paragraph::new(format!("\u{2022} {}", &elem.text))
                        .styled(style),
                );
            }
            ElementKind::Normal => {
                doc.push(
                    genpdf::elements::Paragraph::new(&elem.text)
                        .padded((0, 2))
                        .styled(style),
                );
            }
        }
    }

    let mut output = Vec::new();
    doc.render(&mut output)
        .map_err(|e| ConversionError::SerializeError(format!("pdf render: {}", e)))?;
    Ok(output)
}

fn load_font_family() -> ConversionResult<genpdf::fonts::FontFamily<genpdf::fonts::FontData>> {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_default();

    let search_paths = [
        exe_dir.join("fonts"),
        exe_dir.join("../fonts"),
        exe_dir.join("../../fonts"),
        std::path::PathBuf::from("fonts"),
        std::path::PathBuf::from("src-tauri/fonts"),
    ];

    let mut font_dir = None;
    for path in &search_paths {
        if path.join("LiberationSans-Regular.ttf").exists() {
            font_dir = Some(path.clone());
            break;
        }
    }
    let font_dir = font_dir.unwrap_or_else(|| search_paths[4].clone());

    let regular_bytes = std::fs::read(font_dir.join("LiberationSans-Regular.ttf"))
        .map_err(|e| ConversionError::FontError(format!("regular font: {}", e)))?;
    let bold_bytes = std::fs::read(font_dir.join("LiberationSans-Bold.ttf"))
        .map_err(|e| ConversionError::FontError(format!("bold font: {}", e)))?;
    let italic_bytes = std::fs::read(font_dir.join("LiberationSans-Italic.ttf"))
        .map_err(|e| ConversionError::FontError(format!("italic font: {}", e)))?;
    let bold_italic_bytes = std::fs::read(font_dir.join("LiberationSans-BoldItalic.ttf"))
        .map_err(|e| ConversionError::FontError(format!("bold-italic font: {}", e)))?;

    fn make_font(bytes: Vec<u8>) -> ConversionResult<genpdf::fonts::FontData> {
        genpdf::fonts::FontData::new(bytes, None)
            .map_err(|e| ConversionError::FontError(format!("font data: {}", e)))
    }

    let family = genpdf::fonts::FontFamily {
        regular: make_font(regular_bytes)?,
        bold: make_font(bold_bytes)?,
        italic: make_font(italic_bytes)?,
        bold_italic: make_font(bold_italic_bytes)?,
    };
    Ok(family)
}

pub fn docx_to_pdf(data: &[u8]) -> ConversionResult<Vec<u8>> {
    let elements = extract_docx_elements(data)?;
    if elements.is_empty() {
        return Err(ConversionError::ParseError("no text content found in document".into()));
    }
    render_docx_to_pdf(&elements)
}

// ═══════════════════════════════════════════════════════════════
// Dispatch
// ═══════════════════════════════════════════════════════════════

pub fn convert_document(
    input: &[u8],
    source: &Format,
    target: &Format,
) -> ConversionResult<Vec<u8>> {
    match (source, target) {
        // DOCX → *
        (Format::Docx, Format::Pdf) => docx_to_pdf(input),
        (Format::Docx, Format::Html) => docx_to_html(input),
        (Format::Docx, Format::Markdown) => docx_to_markdown(input),

        // Markdown → *
        (Format::Markdown, Format::Html) => {
            let s = std::str::from_utf8(input).map_err(|e| ConversionError::Utf8Error(e.to_string()))?;
            Ok(markdown_to_html(s).into_bytes())
        }
        (Format::Markdown, Format::Pdf) => {
            let s = std::str::from_utf8(input).map_err(|e| ConversionError::Utf8Error(e.to_string()))?;
            markdown_to_pdf(s)
        }

        // HTML → *
        (Format::Html, Format::Pdf) => {
            let s = std::str::from_utf8(input).map_err(|e| ConversionError::Utf8Error(e.to_string()))?;
            html_to_pdf(s)
        }
        (Format::Html, Format::Markdown) => {
            let s = std::str::from_utf8(input).map_err(|e| ConversionError::Utf8Error(e.to_string()))?;
            Ok(html_to_markdown(s).into_bytes())
        }

        _ => Err(ConversionError::UnsupportedConversion {
            from: source.to_string(),
            to: target.to_string(),
        }),
    }
}

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_markdown_to_html_basic() {
        let md = "# Hello\n\nThis is **bold** text.";
        let html = markdown_to_html(md);
        assert!(html.contains("<h1>"));
        assert!(html.contains("Hello"));
        assert!(html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn test_markdown_to_html_list() {
        let md = "- Item 1\n- Item 2\n";
        let html = markdown_to_html(md);
        assert!(html.contains("<ul>"));
        assert!(html.contains("<li>"));
    }

    #[test]
    fn test_extract_docx_basic() {
        let docx = make_minimal_docx("Hello World");
        let elements = extract_docx_elements(&docx).unwrap();
        assert!(!elements.is_empty());
        assert_eq!(elements[0].text.trim(), "Hello World");
    }

    #[test]
    fn test_docx_to_html() {
        let docx = make_minimal_docx("Hello World");
        let html = docx_to_html(&docx).unwrap();
        let html_str = String::from_utf8(html).unwrap();
        // The test docx has <w:b/> (bold), so output wraps in <strong>
        assert!(html_str.contains("Hello World"));
        assert!(html_str.contains("<strong>"));
    }

    #[test]
    fn test_docx_to_markdown() {
        let docx = make_minimal_docx("Hello World");
        let md = docx_to_markdown(&docx).unwrap();
        let md_str = String::from_utf8(md).unwrap();
        assert!(md_str.contains("Hello World"));
    }

    #[test]
    fn test_html_to_markdown() {
        let html = "<html><body><h1>Title</h1><p>Hello <strong>World</strong></p></body></html>";
        let md = html_to_markdown(html);
        assert!(md.contains("# Title"));
        assert!(md.contains("Hello World"));
    }

    #[test]
    #[ignore = "requires Liberation Sans fonts"]
    fn test_docx_to_pdf_roundtrip() {
        let docx = make_minimal_docx("Test Document Content");
        let pdf = docx_to_pdf(&docx).unwrap();
        assert!(!pdf.is_empty());
        assert!(pdf.starts_with(b"%PDF"));
    }

    #[test]
    #[ignore = "requires Liberation Sans fonts"]
    fn test_html_to_pdf() {
        let html = "<html><body><h1>Title</h1><p>Paragraph text</p></body></html>";
        let pdf = html_to_pdf(html).unwrap();
        assert!(!pdf.is_empty());
        assert!(pdf.starts_with(b"%PDF"));
    }

    #[test]
    #[ignore = "requires Liberation Sans fonts"]
    fn test_markdown_to_pdf() {
        let md = "# Title\n\nParagraph with **bold** text.";
        let pdf = markdown_to_pdf(md).unwrap();
        assert!(!pdf.is_empty());
        assert!(pdf.starts_with(b"%PDF"));
    }

    fn make_minimal_docx(text: &str) -> Vec<u8> {
        let document_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:r><w:rPr><w:b/><w:sz w:val="24"/></w:rPr><w:t>{}</w:t></w:r>
    </w:p>
  </w:body>
</w:document>"#,
            text
        );

        let rels_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"></Relationships>"#;

        let content_types_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>"#;

        let mut zip_buf = Vec::new();
        {
            let mut zip_writer = zip::ZipWriter::new(Cursor::new(&mut zip_buf));
            let options = zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated);
            zip_writer.start_file("[Content_Types].xml", options).unwrap();
            zip_writer.write_all(content_types_xml.as_bytes()).unwrap();
            zip_writer.start_file("_rels/.rels", options).unwrap();
            zip_writer.write_all(rels_xml.as_bytes()).unwrap();
            zip_writer.start_file("word/document.xml", options).unwrap();
            zip_writer.write_all(document_xml.as_bytes()).unwrap();
            zip_writer.finish().unwrap();
        }
        zip_buf
    }
}
