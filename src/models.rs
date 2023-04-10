use color_eyre::eyre::eyre;
use hashbrown::HashMap;
use parking_lot::RwLock;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct DiscoveredBulb {
    pub ip_address: String,
    pub mac_address: String,
}

impl DiscoveredBulb {
    pub fn new(ip: String, mac: String) -> Self {
        Self {
            ip_address: ip,
            mac_address: mac,
        }
    }
}

pub struct BulbRegistry {
    bulbs_by_mac: RwLock<HashMap<String, DiscoveredBulb>>,
}

impl BulbRegistry {
    pub fn new() -> Self {
        Self {
            bulbs_by_mac: RwLock::new(HashMap::new()),
        }
    }
    pub fn register(&self, bulb: DiscoveredBulb) {
        let mut w = self.bulbs_by_mac.write();
        w.insert(bulb.mac_address.clone(), bulb);
    }
    pub fn bulbs(&self) -> Vec<&DiscoveredBulb> {
        let r = self.bulbs_by_mac.read();
        r.par_values().collect::<Vec<&DiscoveredBulb>>()
    }
    pub fn inner(&self) -> &HashMap<String, DiscoveredBulb> {
        &self.bulbs_by_mac.read()
    }
    pub fn into_inner(self) -> HashMap<String, DiscoveredBulb> {
        self.bulbs_by_mac.into_inner()
    }
    pub fn is_registered(&self, mac: &str) -> bool {
        self.bulbs_by_mac.read().contains_key(mac)
    }
    pub fn get(&self, mac: &str) -> Option<&DiscoveredBulb> {
        self.bulbs_by_mac.read().get(mac)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct BulbRegistration {
    pub mac: String,
    pub success: bool,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RegistrationMessage {
    pub method: String,
    pub env: String,
    pub result: BulbRegistration,
    #[serde(skip)]
    pub ip: Option<SocketAddr>,
}

impl TryInto<DiscoveredBulb> for RegistrationMessage {
    type Error = color_eyre::Report;
    fn try_into(self) -> Result<DiscoveredBulb, Self::Error> {
        if !self.result.success {
            return Err(eyre!("Registration result failed"));
        }
        let ip = self.ip.ok_or(eyre!("No ip address"))?.ip().to_string();
        Ok(DiscoveredBulb {
            ip_address: ip,
            mac_address: self.result.mac,
        })
    }
}
