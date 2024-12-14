use day_14::part2::process;
use anyhow::Context;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "day_14")]
pub struct Args {
    #[clap(short, long, default_value = "input2.txt")]
    input: String,
}

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let _args = Args::parse();

    let result = process(include_bytes!("../../input2.txt")).context("process part 2")?;
    println!("{}", result);
    Ok(())
}
