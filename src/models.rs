use hashbrown::HashMap;
use parking_lot::RwLock;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::WizError;
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

impl Default for BulbRegistry {
    fn default() -> Self {
        Self {
            bulbs_by_mac: RwLock::new(HashMap::new()),
        }
    }
}

impl BulbRegistry {
    pub fn register(&self, bulb: DiscoveredBulb) {
        let mut w = self.bulbs_by_mac.write();
        w.insert(bulb.mac_address.clone(), bulb);
    }
    pub fn bulbs(&self) -> Vec<DiscoveredBulb> {
        let r = self.bulbs_by_mac.read();
        r.par_values().cloned().collect::<Vec<DiscoveredBulb>>()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BulbRegistration {
    pub mac: String,
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegistrationMessage {
    pub method: String,
    pub env: String,
    pub result: BulbRegistration,
    #[serde(skip)]
    pub ip: Option<SocketAddr>,
}

impl TryInto<DiscoveredBulb> for RegistrationMessage {
    type Error = WizError;
    fn try_into(self) -> Result<DiscoveredBulb, Self::Error> {
        if !self.result.success {
            return Err(WizError::RegErr(self));
        }
        let ip = self
            .ip
            .ok_or(WizError::NoIP(self.clone()))?
            .ip()
            .to_string();
        Ok(DiscoveredBulb {
            ip_address: ip,
            mac_address: self.result.mac,
        })
    }
}
