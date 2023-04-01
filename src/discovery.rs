use crate::models::{BulbRegistry, DiscoveredBulb};
use crate::utils::{create_udp_broadcast, get_local_adddrs};
use color_eyre::eyre::eyre;
use color_eyre::Result;
use rayon::prelude::*;
use serde_json::to_vec;
use socket2::{SockAddr, Socket};
use std::io::Read;
use std::mem::MaybeUninit;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub const PORT: u16 = 38899;
pub const DEFAULT_WAIT_TIME: f64 = 5.0;
pub const REGISTER_MESSAGE: &str = r#"{"method":"registration","params":{"phoneMac":"AAAAAAAAAAAA","register":false,"phoneIp":"1.2.3.4","id":"1"}}"#;

pub struct BroadcastProtocol<'a> {
    reg: BulbRegistry<'a>,
    broadcast_addr: SocketAddr,
    transport: Socket,
}

impl<'a> BroadcastProtocol<'a> {
    pub fn new(reg: BulbRegistry<'a>, addr: &str) -> Result<Self> {
        let broadcast_addr = addr.parse::<SocketAddr>()?;
        let transport = create_udp_broadcast(38899)?;
        let reg = BulbRegistry::new();
        Ok(Self {
            reg,
            broadcast_addr,
            transport,
        })
    }
    pub fn discover(&mut self) -> Result<serde_json::Value> {
        let addr = SockAddr::from(self.broadcast_addr);
        self.transport.send_to(REGISTER_MESSAGE.as_bytes(), &addr)?;
        let our = get_local_adddrs();
        println!("{:?}", our);
        loop {
            let buf = &mut [0u8; 100];
            let buf1 = unsafe { &mut *(buf as *mut [u8] as *mut [MaybeUninit<u8>]) };
            let (n, addr) = self.transport.recv_from(buf1)?;
            if addr.is_ipv4() {
                let ad = addr.as_socket_ipv4().unwrap().ip().to_string();
                if !our.contains(&ad) {
                    println!("Received {} bytes from {}", n, ad);
                    // return Ok(buf
                    //     .split(|x| x == &b'\0')
                    //     .next()
                    //     .ok_or(eyre!("No not null bytes"))?
                    //     .to_vec());
                    let resp: serde_json::Value = serde_json::from_slice(&buf[0..n])?;
                    let mac = resp
                        .as_object()
                        .ok_or(eyre!("Not an object"))?
                        .get("result")
                        .and_then(|x| x.as_object())
                        .and_then(|x| x.get("mac").and_then(|x| x.as_str()).map(|x| x.to_string()))
                        .ok_or(eyre!("Failed to get mac"))?;
                    println!("Discovered bulb with IP {ad} and MAC: {mac}");
                    let to_reg = DiscoveredBulb::new(&ad, &mac);
                    self.reg.register(to_reg);
                    return Ok(resp);
                }
            }
        }
    }
}
