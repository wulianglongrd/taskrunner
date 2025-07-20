use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Task {
    pub name: String,
    pub command: String,
    pub output_type: Option<String>,
    pub output_context: Option<serde_yaml::Value>,
    pub add_to_context: Option<HashMap<String, serde_yaml::Value>>,
    pub skip_if: Option<Vec<Condition>>,
    pub check_ok: Option<Vec<Condition>>,
    pub when_ok: Option<Vec<Task>>,
    pub when_err: Option<Vec<Task>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Condition {
    pub key: String,
    pub string_match: StringMatcher,
}

// see https://www.envoyproxy.io/docs/envoy/latest/api-v3/type/matcher/v3/string.proto
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct StringMatcher {
    pub exact: Option<String>,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub contains: Option<String>,
    pub safe_regex: Option<String>,
    pub ignore_case: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TaskList {
    pub tasks: Vec<Task>,
} 