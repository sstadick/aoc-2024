use day_04::part2::process;
use anyhow::Context;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "day_04")]
pub struct Args {
    #[clap(short, long, default_value = "input2.txt")]
    input: String,
}

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let result = process(&args.input).context("process part 2")?;
    println!("{}", result);
    Ok(())
}
