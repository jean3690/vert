use std::path::Path;
use std::process;

use vert_lib::converter;
use vert_lib::error::ConversionError;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => help(),
        2 => match args[1].as_str() {
            "--help" | "-h" => help(),
            "--list" => list(),
            input => show_targets(input),
        },
        _ => {
            let input = &args[1];
            let target = &args[2];
            let output = parse_output_arg(&args);

            if let Err(e) = convert(input, target, output.as_deref()) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn parse_output_arg(args: &[String]) -> Option<String> {
    if args.len() >= 5 && args[3] == "-o" {
        Some(args[4].clone())
    } else {
        None
    }
}

fn convert(input: &str, target_str: &str, output: Option<&str>) -> Result<(), ConversionError> {
    let source = converter::Format::from_extension(input)
        .ok_or_else(|| ConversionError::InvalidFormat(format!(
            "cannot detect format from filename: {}",
            input
        )))?;

    let target = converter::Format::from_str(target_str)?;

    // Pre-flight: check that the conversion is supported
    let valid = source.valid_targets();
    if !valid.contains(&target) {
        eprintln!(
            "Error: {} cannot be converted to {} (valid targets: {})",
            source,
            target,
            valid.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(", "),
        );
        std::process::exit(1);
    }

    let output_path = match output {
        Some(p) => p.to_string(),
        None => {
            let p = Path::new(input);
            p.with_extension(target.extension())
                .to_string_lossy()
                .to_string()
        }
    };

    let data = std::fs::read(input)?;
    let result = converter::convert_file(&data, &source, &target)?;
    std::fs::write(&output_path, &result)?;

    println!("{} -> {}", input, output_path);
    Ok(())
}

fn show_targets(input: &str) {
    match converter::Format::from_extension(input) {
        Some(fmt) => {
            println!("{} — {}", fmt, input);
            let targets = fmt.valid_targets();
            if targets.is_empty() {
                println!("  (no conversions available)");
            } else {
                for t in &targets {
                    println!("  -> {}", t);
                }
            }
        }
        None => {
            eprintln!("Cannot detect format from: {}", input);
            eprintln!("Use --list to see supported formats.");
            process::exit(1);
        }
    }
}

fn list() {
    let categories: &[(&str, &[converter::Format])] = &[
        ("Config", &[
            converter::Format::Properties,
            converter::Format::Yaml,
            converter::Format::Json,
            converter::Format::Toml,
            converter::Format::Xml,
            converter::Format::Ini,
        ]),
        ("Data", &[
            converter::Format::Csv,
        ]),
        ("Document", &[
            converter::Format::Epub,
            converter::Format::Docx,
            converter::Format::Pdf,
            converter::Format::Markdown,
            converter::Format::Html,
            converter::Format::Txt,
        ]),
        ("Image", &[
            converter::Format::Svg,
            converter::Format::Png,
            converter::Format::Jpeg,
            converter::Format::Gif,
            converter::Format::Webp,
            converter::Format::Bmp,
            converter::Format::Tiff,
        ]),
    ];

    for (cat, fmts) in categories {
        println!("{}:", cat);
        for f in *fmts {
            let targets: Vec<String> = f.valid_targets().iter().map(|t| t.to_string()).collect();
            println!("  {:<12} .{:<8} -> {}", f.to_string(), f.extension(), targets.join(", "));
        }
    }
}

fn help() {
    println!("Vert — file format converter (CLI)");
    println!();
    println!("Usage:");
    println!("  vert <input> <format>          Convert file to target format");
    println!("  vert <input> <format> -o <out> Specify output path");
    println!("  vert <input>                   Show valid target formats for a file");
    println!("  vert --list                    List all supported formats");
    println!("  vert --help                    Show this help");
    println!();
    println!("Examples:");
    println!("  vert data.json yaml");
    println!("  vert config.xml json -o out.json");
    println!("  vert document.docx pdf");
}
