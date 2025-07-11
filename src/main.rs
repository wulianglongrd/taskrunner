use clap::Parser;

mod runner;
mod config;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "tasks.yaml")]
    config: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    runner::run_tasks(&args.config)?;
    Ok(())
}
