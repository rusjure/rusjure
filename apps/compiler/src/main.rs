use clap::Parser;
use log::info;

#[derive(Parser, Debug)]
struct Args {}

fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .try_init()?;

    let _args: Args = Args::parse();

    info!("Starting up...");

    Ok(())
}
