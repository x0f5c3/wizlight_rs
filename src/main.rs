mod discovery;
mod models;
mod utils;

use crate::discovery::PORT;
use discovery::BroadcastProtocol;
use models::BulbRegistry;

fn main() {
    let brod_addr = format!("255.255.255.255:{}", PORT);
    let mut proto = BroadcastProtocol::new(BulbRegistry::new(), &brod_addr).unwrap();
    let disco = proto.discover().unwrap();
    println!("{:?}", disco);
}
