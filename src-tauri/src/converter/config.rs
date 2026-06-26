use crate::converter::traits::{ConversionResult, Format};
use crate::error::ConversionError;
use serde_json::Value;

// ── Parse functions ──

fn properties_to_value(input: &str) -> ConversionResult<Value> {
    // Manual parsing of .properties format (key=value, one per line)
    let mut map = serde_json::Map::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('!') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = value.trim().to_string();
            map.insert(key, Value::String(value));
        } else if let Some((key, _)) = line.split_once(':') {
            let key = key.trim().to_string();
            let value = line[key.len() + 1..].trim().to_string();
            map.insert(key, Value::String(value));
        }
    }
    Ok(Value::Object(map))
}

fn yaml_to_value(input: &str) -> ConversionResult<Value> {
    serde_yaml::from_str(input)
        .map_err(|e| ConversionError::ParseError(format!("yaml: {}", e)))
}

fn json_to_value(input: &str) -> ConversionResult<Value> {
    serde_json::from_str(input)
        .map_err(|e| ConversionError::ParseError(format!("json: {}", e)))
}

fn toml_to_value(input: &str) -> ConversionResult<Value> {
    toml::from_str(input)
        .map_err(|e| ConversionError::ParseError(format!("toml: {}", e)))
}

fn xml_to_value(input: &str) -> ConversionResult<Value> {
    quick_xml::de::from_str(input)
        .map_err(|e| ConversionError::ParseError(format!("xml: {}", e)))
}

// ── Serialize functions ──

fn value_to_properties(value: &Value) -> ConversionResult<String> {
    let map = value
        .as_object()
        .ok_or_else(|| ConversionError::SerializeError("properties requires an object".into()))?;

    let mut output = String::new();
    for (key, val) in map {
        let v = match val {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        };
        output.push_str(&format!("{}={}\n", key, v));
    }
    Ok(output)
}

fn value_to_yaml(value: &Value) -> ConversionResult<String> {
    serde_yaml::to_string(value)
        .map_err(|e| ConversionError::SerializeError(format!("yaml: {}", e)))
}

fn value_to_json(value: &Value) -> ConversionResult<String> {
    serde_json::to_string_pretty(value)
        .map_err(|e| ConversionError::SerializeError(format!("json: {}", e)))
}

fn value_to_toml(value: &Value) -> ConversionResult<String> {
    toml::to_string(value)
        .map_err(|e| ConversionError::SerializeError(format!("toml: {}", e)))
}

fn value_to_xml(value: &Value) -> ConversionResult<String> {
    quick_xml::se::to_string_with_root("root", value)
        .map_err(|e| ConversionError::SerializeError(format!("xml: {}", e)))
}

// ── Dispatch ──

pub fn convert_config(
    input: &str,
    source: &Format,
    target: &Format,
) -> ConversionResult<String> {
    let value = match source {
        Format::Properties => properties_to_value(input)?,
        Format::Yaml => yaml_to_value(input)?,
        Format::Json => json_to_value(input)?,
        Format::Toml => toml_to_value(input)?,
        Format::Xml => xml_to_value(input)?,
        _ => {
            return Err(ConversionError::UnsupportedConversion {
                source: source.to_string(),
                target: target.to_string(),
            });
        }
    };

    match target {
        Format::Properties => value_to_properties(&value),
        Format::Yaml => value_to_yaml(&value),
        Format::Json => value_to_json(&value),
        Format::Toml => value_to_toml(&value),
        Format::Xml => value_to_xml(&value),
        _ => Err(ConversionError::UnsupportedConversion {
            source: source.to_string(),
            target: target.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_yaml_roundtrip() {
        let json = r#"{"name":"test","count":42}"#;
        let value = json_to_value(json).unwrap();
        let yaml = value_to_yaml(&value).unwrap();
        let back = yaml_to_value(&yaml).unwrap();
        assert_eq!(value, back);
    }

    #[test]
    fn test_properties_to_json() {
        let props = "name=test\ncount=42\n";
        let value = properties_to_value(props).unwrap();
        assert_eq!(value["name"], Value::String("test".into()));
        assert_eq!(value["count"], Value::String("42".into()));
    }

    #[test]
    fn test_json_to_properties() {
        let json = r#"{"name":"test","count":"42"}"#;
        let value = json_to_value(json).unwrap();
        let props = value_to_properties(&value).unwrap();
        assert!(props.contains("name=test"));
        assert!(props.contains("count=42"));
    }

    #[test]
    fn test_toml_json_roundtrip() {
        let toml_str = r#"name = "test"
count = 42
"#;
        let value = toml_to_value(toml_str).unwrap();
        let json = value_to_json(&value).unwrap();
        let back = json_to_value(&json).unwrap();
        assert_eq!(value, back);
    }

    #[test]
    fn test_xml_to_json() {
        let xml = r#"<root><name>test</name><count>42</count></root>"#;
        let value = xml_to_value(xml).unwrap();
        // XML with quick-xml serialize feature wraps in a root object
        assert!(value.is_object());
    }

    #[test]
    fn test_json_to_xml() {
        let json = r#"{"name":"test","count":"42"}"#;
        let value = json_to_value(json).unwrap();
        let xml = value_to_xml(&value).unwrap();
        assert!(xml.contains("<root>"));
    }

    #[test]
    fn test_properties_roundtrip_via_json() {
        let props = "hello=world\nfoo=bar\n";
        let value = properties_to_value(props).unwrap();
        let back = value_to_properties(&value).unwrap();
        assert!(back.contains("hello=world"));
        assert!(back.contains("foo=bar"));
    }

    #[test]
    fn test_invalid_json() {
        let result = json_to_value("not json");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_yaml() {
        let result = yaml_to_value("\tbad indent");
        assert!(result.is_err());
    }
}
