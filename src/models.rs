use hashbrown::HashMap;
use parking_lot::RwLock;
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct DiscoveredBulb {
    ip_address: String,
    mac_address: String,
}

impl DiscoveredBulb {
    pub fn new(ip: String, mac: String) -> Self {
        Self {
            ip_address: ip,
            mac_address: mac,
        }
    }
}

pub struct BulbRegistry<'a> {
    bulbs_by_mac: RwLock<HashMap<&'a str, DiscoveredBulb>>,
}

impl<'a> BulbRegistry<'a> {
    pub fn new() -> Self {
        Self {
            bulbs_by_mac: RwLock::new(HashMap::new()),
        }
    }
    pub fn register(&self, bulb: DiscoveredBulb) {
        let mut w = self.bulbs_by_mac.write();
        w.insert(&bulb.mac_address, bulb);
    }
    pub fn bulbs(&self) -> Vec<DiscoveredBulb> {
        let r = self.bulbs_by_mac.read();
        r.par_values().cloned().collect::<Vec<DiscoveredBulb>>()
    }
}
