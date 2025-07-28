use log::LevelFilter;

mod download;

fn main() -> anyhow::Result<()> {
    simple_logger::SimpleLogger::new()
        .env()
        .with_level(LevelFilter::Off)
        .with_module_level("automation", LevelFilter::Debug)
        .init()
        .expect("Failed to start logger");

    Ok(())
}