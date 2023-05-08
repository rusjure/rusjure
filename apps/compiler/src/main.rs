use log::info;

fn main() -> anyhow::Result<()> {
    env_logger::builder()
    .filter_level(log::LevelFilter::Info)
    .parse_default_env()
    .try_init()?;

    info!("Starting up...");

    Ok(())
}
