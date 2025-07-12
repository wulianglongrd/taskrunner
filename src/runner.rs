use std::collections::HashMap;
use std::fs;
use std::process::Command;
use handlebars::Handlebars;
use jsonpath_lib::select;
use serde_json::Value;
use crate::config::{Config, Task};

pub fn run_tasks(path: &str) -> anyhow::Result<()> {
    let content = fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;

    let mut context: HashMap<String, String> = HashMap::new();
    for task in config.tasks {
        run_task(task, &mut context)?
    }

    Ok(())
}

pub fn run_task(task: Task, context: &mut HashMap<String, String>) -> anyhow::Result<()> {
    println!("===> Running task {}", task.name);
    // println!("{:?}", task);
    println!("Context: {:?}", context);

    let handlebars = Handlebars::new();
    let rendered = handlebars.render_template(&task.command, &context)?;
    println!("$ {}", rendered);

    let output = Command::new("sh").arg("-c").arg(&rendered).output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    //let stderr = String::from_utf8_lossy(&output.stderr);
    println!("{}", stdout);

    add_to_context(&task, context, &stdout)?;

    let mut is_ok = false;
    if let Some(ok_match) = task.check_ok {
        let context_match = if ok_match.context.is_empty() {
            true
        } else {
            ok_match.context.iter().all(|(key, matcher)| {
                context.get(key).map_or(false, |value| matcher.is_match(value))
            })
        };
        let output_match = if ok_match.output.is_empty() {
            true
        } else {
           ok_match.output.iter().all(|matcher| {
               matcher.is_match(&stdout)
           })
        };
        is_ok = context_match && output_match;
    };

    if is_ok && task.when_ok.is_some(){
        run_task(*task.when_ok.unwrap(), context)?;
    }

    if !is_ok && task.when_err.is_some() {
        run_task(*task.when_err.unwrap(), context)?;
    }

    Ok(())
}

fn add_to_context(task: &Task, context: &mut HashMap<String, String>, stdout: &str) -> anyhow::Result<()> {
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
    Ok(())
}