use crate::task::{Task, TaskList};
use crate::context::Context;
use crate::matcher::matches;
use crate::parser::DataExtractor;
use crate::config::{ExtractRule, OutputType};
use serde_yaml;
use std::fs;
use anyhow::{Result, anyhow};
use crate::Args;

pub fn run_with_config(args: &Args) -> Result<()> {
    let yaml = fs::read_to_string(&args.config)?;
    let task_list: TaskList = serde_yaml::from_str(&yaml)?;

    let mut ctx = Context::new();
    for task in &task_list.tasks {
        run_task(task, &mut ctx)?;
    }
    println!("Final context: {}", ctx.data);
    Ok(())
}

fn run_task(task: &Task, ctx: &mut Context) -> Result<()> {
    println!("===> Running task {}", task.name);
    // println!("{:?}", task);
    println!("Context: {:?}\n", ctx.data);

    // 0. 处理 add_to_context
    if let Some(adds) = &task.add_to_context {
        for (k, v) in adds {
            // serde_yaml::Value -> serde_json::Value
            let json_val = serde_json::to_value(v).unwrap_or(serde_json::Value::Null);
            ctx.set(k, json_val);
        }
    }

    // 1. 判断 skip_if 条件（支持多个），在 command 之前
    let mut should_skip = false;
    if let Some(conds) = &task.skip_if {
        for cond in conds {
            let val = ctx.get(&cond.key).and_then(|v| v.as_str().map(|s| s.to_string()).or_else(|| Some(v.to_string())));
            if let Some(val) = val {
                if matches(&val, &cond.string_match) {
                    should_skip = true;
                    break;
                }
            }
        }
    }
    // skip_if 不存在时默认不跳过
    if should_skip {
        println!("Task [{}] skipped by skip_if", task.name);
        return Ok(());
    }

    // 2. 渲染命令模板
    let cmd_str = ctx.render(&task.command)?;
    println!("Executing [{}]: {}", task.name, cmd_str);

    // 3. 执行命令
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(&cmd_str)
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    println!("Output: {}", stdout.trim());

    // 4. 解析输出，按 output_context.extract 规则提取并写入 context
    if let Some(ctx_obj) = &task.output_context {
        // 只支持 extract 规则
        if let Some(rules) = ctx_obj.get("extract").and_then(|v| v.as_sequence()) {
            // 反序列化 ExtractRule
            let rules: Vec<ExtractRule> = serde_yaml::from_value(serde_yaml::Value::Sequence(rules.clone())).map_err(|e| anyhow!(e))?;
            // 解析 output_type
            let output_type = if let Some(t) = &task.output_type {
                serde_yaml::from_str::<OutputType>(t).ok()
            } else { None };
            let extracted = DataExtractor::extract_data(&stdout, &rules, output_type.as_ref(), None)?;
            for (k, v) in extracted {
                ctx.set(&k, serde_json::json!(v));
            }
        }
    }

    // 5. 判断 check_ok 条件（支持多个），在 output_context 之后
    let mut is_ok = true;
    if let Some(conds) = &task.check_ok {
        for cond in conds {
            let val = ctx.get(&cond.key).and_then(|v| v.as_str().map(|s| s.to_string()).or_else(|| Some(v.to_string())));
            if let Some(val) = val {
                if !matches(&val, &cond.string_match) {
                    is_ok = false;
                    break;
                }
            } else {
                is_ok = false;
                break;
            }
        }
    }
    // 6. 根据 check_ok 结果递归执行 when_ok/when_err
    // check_ok 不存在时默认 is_ok = true
    if is_ok {
        if let Some(subtasks) = &task.when_ok {
            for sub in subtasks {
                run_task(sub, ctx)?;
            }
        }
    } else {
        if let Some(subtasks) = &task.when_err {
            for sub in subtasks {
                run_task(sub, ctx)?;
            }
        }
    }
    Ok(())
} 