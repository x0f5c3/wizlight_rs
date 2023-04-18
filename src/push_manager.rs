use crate::models::DiscoveredBulb;
use crate::Result;
use crate::WizError;
use hashbrown::HashMap;
use itertools::Itertools;
use once_cell::sync::Lazy;
use rand::prelude::SliceRandom;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::net::UdpSocket;

const RESPOND_PORT: i32 = 38899;
const LISTEN_PORT: i32 = 38900;

static MAC_CHARS: Lazy<Vec<String>> = Lazy::new(|| {
    vec![
        "0".to_string(),
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
        "6".to_string(),
        "7".to_string(),
        "8".to_string(),
        "9".to_string(),
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
        "e".to_string(),
        "f".to_string(),
    ]
});

#[derive(Debug, Deserialize, Serialize)]
pub struct RegParams {
    #[serde(rename = "phoneIp")]
    phone_ip: String,
    register: bool,
    #[serde(rename = "phoneMac")]
    phone_mac: String,
}

impl RegParams {
    pub fn new(phone_ip: String, register: bool, phone_mac: String) -> Self {
        Self {
            phone_ip,
            register,
            phone_mac,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PushRegisterMessage {
    params: RegParams,
    method: String,
}

impl PushRegisterMessage {
    pub fn new(target_ip: &str) -> Result<Self> {
        let sock = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
        let conn_ip = SockAddr::from(SocketAddr::from_str(target_ip)?);
        sock.connect(&conn_ip)?;
        let sock_addr = sock.local_addr()?;
        let ip = sock_addr
            .as_socket_ipv4()
            .ok_or(WizError::IP6(sock_addr))?
            .ip()
            .to_string();
        let mac = gen_mac();
        let params = RegParams::new(ip, true, mac);
        Ok(Self {
            params,
            method: "registration".to_string(),
        })
    }
}

pub fn gen_mac() -> String {
    MAC_CHARS.choose_multiple(&mut OsRng, 12).join("")
}

pub struct PushManager<F: Fn(DiscoveredBulb), FS: Fn(serde_json::Value, &str)> {
    transport: UdpSocket,
    push_running: bool,
    discovery_callback: Option<F>,
    reg_message: PushRegisterMessage,
    subs: HashMap<String, FS>,
}
