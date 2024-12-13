use {{crate_name}}::part1::process;
use anyhow::Context;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "{{crate_name}}")]
pub struct Args {
    #[clap(short, long, default_value = "input1.txt")]
    input: String,
}

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let _args = Args::parse();

    let result = process(include_bytes!("../../input1.txt")).context("process part 1")?;
    println!("{}", result);
    Ok(())
}
