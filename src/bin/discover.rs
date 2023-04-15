use wizlight_rs::discovery::BroadcastProtocol;
use wizlight_rs::Result;

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
        .try_init()?;
    info!("Initialized subscriber");
    let mut proto = BroadcastProtocol::new(None)?;
    proto.discover().await?;
    info!("{:?}", proto.reg.bulbs());
    Ok(())
}
