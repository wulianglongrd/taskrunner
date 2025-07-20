mod task;
mod context;
mod matcher;
mod parser;
mod config;
mod runner;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the YAML config file
    #[arg(short, long, default_value = "tasks.yaml")]
    pub config: String,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = runner::run_with_config(&args) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
