use crate::bulblibrary::BulbClass;
use crate::discovery::BroadcastProtocol;
use buildstructor::buildstructor;
use hashbrown::HashMap;
use std::sync::Arc;

pub struct WizLight {
    ip: String,
    port: u32,
    mac: String,
    bulb_type: BulbClass,
    model_config: HashMap<String, String>,
    white_range: Vec<f64>,
    ext_white_range: Vec<f64>,
    transport: Arc<BroadcastProtocol>,
}
