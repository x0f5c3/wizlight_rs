#![allow(dead_code)]
mod bulb;
mod bulblibrary;
mod cli;
mod discovery;
mod errors;
mod models;
mod protocol;
mod rgbcw;
mod scenes;
mod utils;

use color_eyre::eyre::eyre;
use color_eyre::Result;
use discovery::BroadcastProtocol;

use tracing::level_filters::LevelFilter;
use tracing::{info, Level};
use tracing_appender::rolling;

#[tokio::main]
async fn main() -> Result<()> {
    let log_f = rolling::hourly("./", "runtime.log");
    let (non_block, _guard) = tracing_appender::non_blocking(log_f);
    tracing_subscriber::fmt::Subscriber::builder()
        .json()
        .with_writer(non_block)
        .with_max_level(LevelFilter::from_level(Level::TRACE))
        .with_level(true)
        .with_line_number(true)
        .with_file(true)
        .try_init()
        .map_err(|e| eyre!("Failed to init subscriber {e}"))?;
    info!("Initialized subscriber");
    let mut proto = BroadcastProtocol::new(None)?;
    proto.discover().await?;
    info!("{:?}", proto.reg.bulbs());
    Ok(())
}
