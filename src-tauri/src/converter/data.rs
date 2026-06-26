use crate::converter::traits::{ConversionResult, Format};
use crate::error::ConversionError;
use serde_json::Value;
use std::collections::HashMap;

type Row = HashMap<String, String>;

// ── CSV ──

fn csv_to_rows(input: &str) -> ConversionResult<Vec<Row>> {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(false)
        .from_reader(input.as_bytes());

    let headers: Vec<String> = reader
        .headers()
        .map_err(|e| ConversionError::ParseError(format!("csv headers: {}", e)))?
        .iter()
        .map(|h| h.to_string())
        .collect();

    let mut rows = Vec::new();
    for result in reader.records() {
        let record = result
            .map_err(|e| ConversionError::ParseError(format!("csv record: {}", e)))?;
        let mut row = Row::new();
        for (i, field) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                row.insert(header.clone(), field.to_string());
            }
        }
        rows.push(row);
    }
    Ok(rows)
}

fn rows_to_csv(rows: &[Row]) -> ConversionResult<String> {
    if rows.is_empty() {
        return Ok(String::new());
    }
    // Collect union of all keys preserving insertion order from first row
    let mut headers = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for row in rows {
        for key in row.keys() {
            if seen.insert(key.clone()) {
                headers.push(key.clone());
            }
        }
    }

    let mut wtr = csv::Writer::from_writer(Vec::new());
    wtr.write_record(&headers)
        .map_err(|e| ConversionError::SerializeError(format!("csv: {}", e)))?;
    for row in rows {
        let record: Vec<&str> = headers.iter().map(|h| row.get(h).map(|s| s.as_str()).unwrap_or("")).collect();
        wtr.write_record(&record)
            .map_err(|e| ConversionError::SerializeError(format!("csv: {}", e)))?;
    }
    wtr.flush()
        .map_err(|e| ConversionError::SerializeError(format!("csv: {}", e)))?;
    let output = String::from_utf8(wtr.into_inner().map_err(|e| ConversionError::SerializeError(format!("csv: {}", e)))?)
        .map_err(|e| ConversionError::Utf8Error(e.to_string()))?;
    Ok(output)
}

// ── JSON (tabular) ──

fn data_json_to_rows(input: &str) -> ConversionResult<Vec<Row>> {
    let value: Value = serde_json::from_str(input)
        .map_err(|e| ConversionError::ParseError(format!("json: {}", e)))?;

    let arr = value
        .as_array()
        .ok_or_else(|| ConversionError::ParseError("data json must be an array of objects".into()))?;

    let mut rows = Vec::new();
    for item in arr {
        let obj = item
            .as_object()
            .ok_or_else(|| ConversionError::ParseError("data json items must be objects".into()))?;
        let mut row = Row::new();
        for (k, v) in obj {
            let val = match v {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            row.insert(k.clone(), val);
        }
        rows.push(row);
    }
    Ok(rows)
}

fn rows_to_data_json(rows: &[Row]) -> ConversionResult<String> {
    let arr: Value = Value::Array(
        rows.iter()
            .map(|row| {
                let map: serde_json::Map<String, Value> = row
                    .iter()
                    .map(|(k, v)| (k.clone(), Value::String(v.clone())))
                    .collect();
                Value::Object(map)
            })
            .collect(),
    );
    serde_json::to_string_pretty(&arr)
        .map_err(|e| ConversionError::SerializeError(format!("json: {}", e)))
}

// ── XML (tabular) ──

fn data_xml_to_rows(input: &str) -> ConversionResult<Vec<Row>> {
    let value: Value = quick_xml::de::from_str(input)
        .map_err(|e| ConversionError::ParseError(format!("xml: {}", e)))?;

    // XML structure: <root><record><field>value</field></record></root>
    // Value is an object with a "record" key that could be an object or array
    let root = value
        .as_object()
        .ok_or_else(|| ConversionError::ParseError("data xml: root must be an object".into()))?;

    let records = if let Some(records_val) = root.get("record") {
        match records_val {
            Value::Array(arr) => arr.clone(),
            Value::Object(_) => vec![records_val.clone()],
            _ => {
                return Err(ConversionError::ParseError(
                    "data xml: <record> not found".into(),
                ))
            }
        }
    } else {
        // Maybe the root itself contains fields directly
        return Err(ConversionError::ParseError(
            "data xml: expected <record> elements".into(),
        ));
    };

    let mut rows = Vec::new();
    for record in &records {
        let obj = record
            .as_object()
            .ok_or_else(|| ConversionError::ParseError("data xml: each <record> must be an object".into()))?;
        let mut row = Row::new();
        for (k, v) in obj {
            let val = match v {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            row.insert(k.clone(), val);
        }
        rows.push(row);
    }
    Ok(rows)
}

fn rows_to_data_xml(rows: &[Row]) -> ConversionResult<String> {
    // Build XML: <root><record>...</record></root>
    let mut xml = String::from("<root>\n");
    for row in rows {
        xml.push_str("  <record>\n");
        for (k, v) in row {
            let escaped = quick_xml::escape::escape(v).to_string();
            xml.push_str(&format!("    <{}>{}</{}>\n", k, escaped, k));
        }
        xml.push_str("  </record>\n");
    }
    xml.push_str("</root>\n");
    Ok(xml)
}

// ── Dispatch ──

pub fn convert_data(input: &str, source: &Format, target: &Format) -> ConversionResult<String> {
    let rows = match source {
        Format::Csv => csv_to_rows(input)?,
        Format::Json => data_json_to_rows(input)?,
        Format::Xml => data_xml_to_rows(input)?,
        _ => {
            return Err(ConversionError::UnsupportedConversion {
                source: source.to_string(),
                target: target.to_string(),
            });
        }
    };

    match target {
        Format::Csv => rows_to_csv(&rows),
        Format::Json => rows_to_data_json(&rows),
        Format::Xml => rows_to_data_xml(&rows),
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
    fn test_csv_to_json() {
        let csv = "name,age\nAlice,30\nBob,25\n";
        let rows = csv_to_rows(csv).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].get("name").unwrap(), "Alice");

        let json = rows_to_data_json(&rows).unwrap();
        assert!(json.contains("Alice"));
        assert!(json.contains("Bob"));
    }

    #[test]
    fn test_csv_to_xml() {
        let csv = "name,age\nAlice,30\n";
        let rows = csv_to_rows(csv).unwrap();
        let xml = rows_to_data_xml(&rows).unwrap();
        assert!(xml.contains("<record>"));
        assert!(xml.contains("<name>Alice</name>"));
    }

    #[test]
    fn test_json_to_csv() {
        let json = r#"[{"name":"Alice","age":"30"},{"name":"Bob","age":"25"}]"#;
        let rows = data_json_to_rows(json).unwrap();
        assert_eq!(rows.len(), 2);
        let csv = rows_to_csv(&rows).unwrap();
        assert!(csv.contains("Alice"));
        assert!(csv.contains("Bob"));
    }

    #[test]
    fn test_csv_roundtrip() {
        let csv = "name,age,city\nAlice,30,NYC\nBob,25,LA\n";
        let rows = csv_to_rows(csv).unwrap();
        let back = rows_to_csv(&rows).unwrap();
        // Should contain same data (order may vary)
        assert!(back.contains("Alice"));
        assert!(back.contains("Bob"));
        assert!(back.contains("NYC"));
    }

    #[test]
    fn test_empty_csv() {
        let csv = "name,age\n";
        let rows = csv_to_rows(csv).unwrap();
        assert!(rows.is_empty());
    }

    #[test]
    fn test_invalid_json_not_array() {
        let json = r#"{"name":"Alice"}"#;
        let result = data_json_to_rows(json);
        assert!(result.is_err());
    }
}
