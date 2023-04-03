use crate::models::{BulbRegistry, DiscoveredBulb};
use crate::utils::{create_udp_broadcast, get_local_adddrs};
use color_eyre::eyre::eyre;
use color_eyre::Result;
use socket2::{SockAddr, Socket};
use std::fs;
use std::mem::MaybeUninit;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tracing::{debug, error, info, instrument, warn};

pub const PORT: u16 = 38899;
pub const DEFAULT_WAIT_TIME: f64 = 5.0;
pub const REGISTER_MESSAGE: &str = r#"{"method":"registration","params":{"phoneMac":"AAAAAAAAAAAA","register":false,"phoneIp":"1.2.3.4","id":"1"}}"#;

pub struct BroadcastProtocol {
    reg: BulbRegistry,
    broadcast_addr: SocketAddr,
    transport: Socket,
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
    pub fn discover(&mut self) -> Result<serde_json::Value> {
        let addr = SockAddr::from(self.broadcast_addr);
        self.transport.send_to(REGISTER_MESSAGE.as_bytes(), &addr)?;
        loop {
            let buf = &mut [0u8; 1024];
            let buf1 = unsafe { &mut *(buf as *mut [u8] as *mut [MaybeUninit<u8>]) };
            let (n, addr) = self.transport.recv_from(buf1)?;
            if addr.is_ipv4() {
                let ad = addr.as_socket_ipv4().unwrap().ip().to_string();
                if !self.local_addrs.contains(&ad) {
                    info!("Received {} bytes from {}", n, ad);
                    let res = fs::write(format!("resp_{}.json", &ad), &buf[0..n]);
                    if let Err(e) = res {
                        error!("Failed to write response to file: {e}");
                    }
                    let resp: serde_json::Value = serde_json::from_slice(&buf[0..n])?;
                    let mac = resp
                        .as_object()
                        .ok_or(eyre!("Not an object"))?
                        .get("result")
                        .and_then(|x| x.as_object())
                        .and_then(|x| x.get("mac").and_then(|x| x.as_str()).map(|x| x.to_string()))
                        .ok_or(eyre!("Failed to get mac"))?;
                    info!("Discovered bulb with IP {ad} and MAC: {mac}");
                    let to_reg = DiscoveredBulb::new(ad, mac);
                    self.reg.register(to_reg);
                    return Ok(resp);
                }
            }
        }
    }
}
