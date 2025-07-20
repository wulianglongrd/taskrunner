use std::collections::HashMap;
use serde_json::Value;
use jsonpath_lib::select;
use crate::config::{ExtractRule, ExtractType, OutputType};

pub struct DataExtractor;

impl DataExtractor {
    pub fn extract_data(
        output: &str,
        rules: &[ExtractRule],
        output_type: Option<&OutputType>,
        separator: Option<&str>,
    ) -> anyhow::Result<HashMap<String, String>> {
        let mut extracted = HashMap::new();
        let parsed_data = Self::parse_output(output, output_type, separator)?;
        for rule in rules {
            let value = Self::extract_field(&parsed_data, &rule.field, rule.extract_type.as_ref())?;
            if let Some(val) = value {
                extracted.insert(rule.as_key.clone(), val);
            }
        }
        Ok(extracted)
    }

    fn parse_output(
        output: &str,
        output_type: Option<&OutputType>,
        separator: Option<&str>,
    ) -> anyhow::Result<Value> {
        let trimmed_output = output.trim();
        match output_type {
            Some(OutputType::Json) => {
                serde_json::from_str(trimmed_output)
                    .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))
            }
            Some(OutputType::Separated) => {
                Self::parse_separated_string(trimmed_output, separator.unwrap_or("||"))
            }
            Some(OutputType::String) | None => {
                if trimmed_output.starts_with('{') && trimmed_output.ends_with('}') {
                    match serde_json::from_str::<Value>(trimmed_output) {
                        Ok(json) => Ok(json),
                        Err(_) => {
                            let mut obj = serde_json::Map::new();
                            obj.insert("value".to_string(), Value::String(trimmed_output.to_string()));
                            Ok(Value::Object(obj))
                        }
                    }
                } else if trimmed_output.contains("||") {
                    Self::parse_separated_string(trimmed_output, "||")
                } else {
                    let mut obj = serde_json::Map::new();
                    obj.insert("value".to_string(), Value::String(trimmed_output.to_string()));
                    Ok(Value::Object(obj))
                }
            }
        }
    }

    fn parse_separated_string(output: &str, separator: &str) -> anyhow::Result<Value> {
        let mut obj = serde_json::Map::new();
        for part in output.split(separator) {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            if let Some((key, value)) = part.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                if value.starts_with('{') && value.ends_with('}') {
                    if let Ok(json_value) = serde_json::from_str::<Value>(value) {
                        obj.insert(key.to_string(), json_value);
                    } else {
                        obj.insert(key.to_string(), Value::String(value.to_string()));
                    }
                } else if value.starts_with('[') && value.ends_with(']') {
                    if let Ok(json_value) = serde_json::from_str::<Value>(value) {
                        obj.insert(key.to_string(), json_value);
                    } else {
                        obj.insert(key.to_string(), Value::String(value.to_string()));
                    }
                } else {
                    if let Ok(num) = value.parse::<f64>() {
                        obj.insert(key.to_string(), Value::Number(serde_json::Number::from_f64(num).unwrap_or(serde_json::Number::from(0))));
                    } else if value == "true" || value == "false" {
                        obj.insert(key.to_string(), Value::Bool(value == "true"));
                    } else {
                        obj.insert(key.to_string(), Value::String(value.to_string()));
                    }
                }
            } else {
                obj.insert(part.to_string(), Value::String("".to_string()));
            }
        }
        Ok(Value::Object(obj))
    }

    fn extract_field(
        data: &Value,
        field: &str,
        extract_type: Option<&ExtractType>,
    ) -> anyhow::Result<Option<String>> {
        let results = select(data, field)?;
        if let Some(val) = results.get(0) {
            let result = match extract_type {
                Some(ExtractType::String) => val.as_str().unwrap_or("").to_string(),
                Some(ExtractType::Number) => {
                    if let Some(n) = val.as_f64() {
                        n.to_string()
                    } else if let Some(n) = val.as_i64() {
                        n.to_string()
                    } else {
                        val.to_string()
                    }
                }
                Some(ExtractType::Boolean) => {
                    if let Some(b) = val.as_bool() {
                        b.to_string()
                    } else {
                        val.to_string()
                    }
                }
                Some(ExtractType::Json) | None => {
                    if val.is_object() || val.is_array() {
                        serde_json::to_string(val)?
                    } else {
                        val.to_string()
                    }
                }
            };
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
} 