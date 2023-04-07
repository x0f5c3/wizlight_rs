use hashbrown::HashMap;
use once_cell::unsync::Lazy;



pub const SCENES: Lazy<HashMap<u32, String>> = Lazy::new(|| {
    let mut res = HashMap::new();
    res.insert(1, "Ocean");
})