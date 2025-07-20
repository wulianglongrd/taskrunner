use serde_json::Value;
use handlebars::Handlebars;
use anyhow::{Result, anyhow};

#[derive(Debug)]
pub struct Context {
    pub data: Value,
    pub hbs: Handlebars<'static>,
}

impl Context {
    pub fn new() -> Self {
        let mut hbs = Handlebars::new();
        hbs.register_escape_fn(handlebars::no_escape);
        Self {
            data: serde_json::json!({}),
            hbs,
        }
    }

    pub fn render(&self, template: &str) -> Result<String> {
        self.hbs.render_template(template, &self.data).map_err(|e| anyhow!(e))
    }

    pub fn set(&mut self, key: &str, value: Value) {
        let mut parts = key.split('.').collect::<Vec<_>>();
        let last = parts.pop().unwrap();
        let mut current = &mut self.data;
        for part in parts {
            current = current
                .as_object_mut()
                .unwrap()
                .entry(part)
                .or_insert_with(|| serde_json::json!({}));
        }
        current[last] = value;
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        let mut current = &self.data;
        for part in key.split('.') {
            current = current.get(part)?;
        }
        Some(current)
    }
} 