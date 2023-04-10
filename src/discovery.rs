use crate::models::{BulbRegistry, DiscoveredBulb, RegistrationMessage};
use crate::utils::{create_udp_broadcast, get_local_adddrs};

use color_eyre::Result;

use color_eyre::eyre::WrapErr;
use indicatif::{ProgressBar, ProgressStyle};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time as tktime;
use tracing::{debug, error, info, instrument, warn};

pub const PORT: u16 = 38899;
pub const DEFAULT_WAIT_TIME: f64 = 5.0;
pub const REGISTER_MESSAGE: &str = r#"{"method":"registration","params":{"phoneMac":"AAAAAAAAAAAA","register":false,"phoneIp":"1.2.3.4","id":"1"}}"#;

pub struct BroadcastProtocol {
    pub reg: BulbRegistry,
    broadcast_addr: SocketAddr,
    transport: UdpSocket,
    local_addrs: Vec<String>,
}

impl BroadcastProtocol {
    #[instrument]
    pub fn new(addr: Option<&str>) -> Result<Self> {
        let broadcast_addr = addr
            .and_then(|x| x.parse::<SocketAddr>().ok())
            .unwrap_or(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::BROADCAST, PORT)));
        let transport = create_udp_broadcast(38899)?;
        debug!("Created the udp socket");
        let reg = BulbRegistry::new();
        Ok(Self {
            reg,
            broadcast_addr,
            transport,
            local_addrs: get_local_adddrs(),
        })
    }
    #[instrument(skip(self))]
    pub async fn recv_from(&self) -> Result<(Vec<u8>, SocketAddr)> {
        let mut buf = [0u8; 1024];
        let (n, addr) = self.transport.recv_from(&mut buf).await?;
        Ok((buf[0..n].to_vec(), addr))
    }
    #[instrument(skip(self))]
    pub async fn recv_foreign(&self) -> Result<(Vec<u8>, SocketAddr)> {
        loop {
            let (buf, addr) = self.recv_from().await?;
            if let SocketAddr::V4(a) = addr {
                let ad = a.ip().to_string();
                if !self.local_addrs.contains(&ad) {
                    info!("Received {} bytes from {}", buf.len(), ad);
                    return Ok((buf, addr));
                }
            }
        }
    }
    #[instrument(skip(self))]
    pub(crate) async fn recv_msg(&self) -> Result<RegistrationMessage> {
        let (b, addr) = self.recv_foreign().await?;
        let mut msg: RegistrationMessage = serde_json::from_slice(b.as_slice())?;
        msg.ip = Some(addr);
        Ok(msg)
    }
    #[instrument(skip(self))]
    pub async fn discover(&mut self) -> Result<()> {
        self.transport
            .send_to(REGISTER_MESSAGE.as_bytes(), self.broadcast_addr)
            .await?;
        let sp = ProgressBar::new_spinner();
        sp.enable_steady_tick(Duration::from_millis(120));
        sp.set_style(
            ProgressStyle::with_template("{spinner:.blue} {msg}")?
                // For more spinners check out the cli-spinners project:
                // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
                .tick_strings(&[
                    "▹▹▹▹▹",
                    "▸▹▹▹▹",
                    "▹▸▹▹▹",
                    "▹▹▸▹▹",
                    "▹▹▹▸▹",
                    "▹▹▹▹▸",
                    "▪▪▪▪▪",
                ]),
        );
        sp.set_message("Discovering...");
        let r: Result<Result<()>> =
            tktime::timeout(Duration::from_secs_f64(DEFAULT_WAIT_TIME), async {
                loop {
                    let resp = self.recv_msg().await?;
                    let to_reg: DiscoveredBulb = resp.try_into()?;
                    info!(
                        "Discovered bulb with IP {} and MAC: {}",
                        to_reg.ip_address, to_reg.mac_address
                    );
                    self.reg.register(to_reg);
                }
            })
            .await
            .wrap_err("Timeout exceeded");
        match r {
            Ok(Err(e)) => error!("Error encountered {e}"),
            Err(e) => warn!("Timeout {e}"),
            _ => {}
        }
        sp.finish_with_message(format!("Discovered {} bulbs", self.reg.bulbs().len()));
        Ok(())
    }
}

#[instrument(skip(buf))]
pub(crate) fn deser_msg(buf: &[u8], addr: SocketAddr) -> Result<RegistrationMessage> {
    let mut resp: RegistrationMessage = serde_json::from_slice(buf)?;
    resp.ip = Some(addr);
    Ok(resp)
}
