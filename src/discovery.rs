use crate::models::{BulbRegistry, DiscoveredBulb, RegistrationMessage};
use crate::utils::{create_udp_broadcast, get_local_adddrs};

use color_eyre::Result;

use std::fs;

use indicatif::{ProgressBar, ProgressStyle};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::time::Duration;
use time::Instant;
use tracing::{debug, error, info, instrument, warn};

pub const PORT: u16 = 38899;
pub const DEFAULT_WAIT_TIME: f64 = 5.0;
pub const REGISTER_MESSAGE: &str = r#"{"method":"registration","params":{"phoneMac":"AAAAAAAAAAAA","register":false,"phoneIp":"1.2.3.4","id":"1"}}"#;

pub struct BroadcastProtocol {
    pub reg: BulbRegistry,
    broadcast_addr: SocketAddr,
    transport: UdpSocket,
    local_addrs: Vec<String>,
    timeout: Duration,
}

impl BroadcastProtocol {
    #[instrument]
    pub fn new(addr: Option<&str>, timeout_m: Option<Duration>) -> Result<Self> {
        let broadcast_addr = addr
            .and_then(|x| x.parse::<SocketAddr>().ok())
            .unwrap_or(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::BROADCAST, PORT)));
        let transport = create_udp_broadcast(38899)?;
        transport.set_read_timeout(timeout_m.or(Some(Duration::from_secs(5))))?;
        let timeout = timeout_m.unwrap_or(Duration::from_secs(5));
        debug!("Created the udp socket");
        let reg = BulbRegistry::new();
        Ok(Self {
            reg,
            broadcast_addr,
            transport,
            local_addrs: get_local_adddrs(),
            timeout,
        })
    }
    #[instrument(skip(self))]
    pub fn recv_from(&mut self) -> Result<(Vec<u8>, SocketAddr)> {
        let buf = &mut [0u8; 1024];
        let (n, addr) = self.transport.recv_from(buf)?;
        Ok((buf[0..n].to_vec(), addr))
    }
    #[instrument(skip(self))]
    pub fn recv_foreign(&mut self) -> Result<(Vec<u8>, SocketAddr)> {
        loop {
            let buf = &mut [0u8; 1024];
            let (n, addr) = self.transport.recv_from(buf)?;
            if let SocketAddr::V4(a) = addr {
                let ad = a.ip().to_string();
                if !self.local_addrs.contains(&ad) {
                    info!("Received {} bytes from {}", n, ad);
                    let res = fs::write(format!("resp_{}.json", &ad), &buf[0..n]);
                    if let Err(e) = res {
                        error!("Failed to write response to file: {e}");
                    }
                    return Ok((buf[0..n].to_vec(), addr));
                }
            }
        }
    }
    #[instrument(skip(self))]
    pub(crate) fn recv_msg(&mut self) -> Result<RegistrationMessage> {
        let (b, addr) = self.recv_foreign()?;
        let mut msg: RegistrationMessage = serde_json::from_slice(b.as_slice())?;
        msg.ip = Some(addr);
        Ok(msg)
    }
    #[instrument(skip(self))]
    pub fn discover(&mut self) -> Result<()> {
        self.transport
            .send_to(REGISTER_MESSAGE.as_bytes(), self.broadcast_addr)?;
        let start = Instant::now();
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
        while start.elapsed().as_seconds_f64() < DEFAULT_WAIT_TIME {
            let buf = &mut [0u8; 1024];
            let (n, addr) = self.transport.recv_from(buf)?;
            if let SocketAddr::V4(a) = addr {
                let ad = a.ip().to_string();
                debug!("Received from {ad}");
                if !self.local_addrs.contains(&ad) {
                    info!("Received {} bytes from {}", n, ad);
                    let res = fs::write(format!("resp_{}.json", &ad), &buf[0..n]);
                    if let Err(e) = res {
                        error!("Failed to write response to file: {e}");
                    }
                    let resp = deser_msg(&buf[..n], addr)?;
                    let to_reg: DiscoveredBulb = resp.try_into()?;
                    info!(
                        "Discovered bulb with IP {} and MAC: {}",
                        to_reg.ip_address, to_reg.mac_address
                    );
                    self.reg.register(to_reg);
                }
            }
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
