#![allow(dead_code)]
mod discovery;
mod models;
mod utils;

use color_eyre::eyre::eyre;
use color_eyre::Result;
use discovery::BroadcastProtocol;
use tracing::level_filters::LevelFilter;
use tracing::{info, Level};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(LevelFilter::from_level(Level::TRACE))
        .with_level(true)
        .with_line_number(true)
        .try_init()
        .map_err(|e| eyre!("Failed to init tracing subscriber {e}"))?;
    info!("Initialized subscriber");
    let mut proto = BroadcastProtocol::new(None, None)?;
    proto.discover()?;
    info!("{:?}", proto.reg.bulbs());
    Ok(())
}
