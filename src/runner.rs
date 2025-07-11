use std::collections::HashMap;
use std::fs;
use std::process::Command;
use handlebars::Handlebars;
use jsonpath_lib::select;
use serde_json::Value;
use crate::config::Config;

pub fn run_tasks(path: &str) -> anyhow::Result<()> {
    let content = fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;

    let mut context: HashMap<String, String> = HashMap::new();
    let handlebars = Handlebars::new();

    for task in config.tasks {
        println!("===> Running task {}", task.name);

        for cmd_template in task.commands  {
            let rendered = handlebars.render_template(&cmd_template, &context)?;
            println!("$ {}", rendered);

            let output = Command::new("sh").arg("-c").arg(&rendered).output()?;
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow::anyhow!("Error running task {}: {}", task.name, stderr));
            }

            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("stdout: {}", stdout);

            if let Some(rules) = &task.extract {
                if let Ok(json) = serde_json::from_str::<Value>(&stdout) {
                    for rule in rules {
                        let results = select(&json, &rule.field)?;
                        if let Some(val) = results.get(0) {
                            let val_str = val.to_string().trim_matches('"').to_string();
                            context.insert(rule.as_key.clone(), val_str.clone());
                            println!("→ Extracted {} = {}", rule.as_key, val_str);
                        }
                    }
                } else {
                    println!("⚠️ Output is not valid JSON, skipping extraction");
                }
            }
        }
    }

    Ok(())
}