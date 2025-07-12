use std::collections::HashMap;
use serde::Deserialize;
use regex::Regex;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Deserialize)]
pub struct Task {
    #[serde(default)]
    pub name: String,
    pub command: String,
    pub extract: Option<Vec<ExtractRule>>,
    #[serde(rename = "okMatch")]
    pub ok_match: Option<OkMatch>,
    #[serde(rename = "whenOk")]
    pub when_ok: Option<Box<Task>>,
    #[serde(rename = "whenErr")]
    pub when_err: Option<Box<Task>>,
}

#[derive(Debug, Deserialize)]
pub struct ExtractRule {
    pub field: String,

    #[serde(rename = "as")]
    pub as_key: String,
}

#[derive(Debug, Deserialize)]
pub struct OkMatch {
    #[serde(default)]
    pub context: HashMap<String, Matcher>,
    #[serde(default)]
    pub output: Vec<Matcher>,
}

#[derive(Debug, Deserialize)]
pub struct Matcher {
    pub exact: Option<String>,
    pub prefix: Option<String>,
    pub regex: Option<String>,
}

impl Matcher {
    pub fn is_match(&self, input: &str) -> bool {
        if let Some(ref exact) = self.exact {
            return input == exact;
        }
        if let Some(ref prefix) = self.prefix {
            return input.starts_with(prefix);
        }
        if let Some(ref regex) = self.regex {
            if let Ok(reg) = Regex::new(regex) {
                return reg.is_match(input);
            }
        }
        false
    }
}