use crate::map;

use hashbrown::HashMap;
use once_cell::unsync::Lazy;

// pub const SCENES: Lazy<HashMap<u32, String>> = Lazy::new(|| {
//     let mut res = HashMap::new();
//     res.insert(1, "Ocean".to_string());
//     res.insert(2, "Romance".to_string());
//     res.insert(3, "Sunset".to_string());
//     res.insert(4, "Party".to_string());
//     res.insert(5, "Fireplace".to_string());
//     res.insert(6, "Cozy".to_string());
//     res.insert(7, "Forest".to_string());
//     res.insert(8, "Pastel Colors".to_string());
//     res.insert(9, "Wake up".to_string());
//     res.insert(10, "Bedtime".to_string());
//     res.insert(11, "Warm White".to_string());
//     res.insert(12, "Daylight".to_string());
//     res.insert(13, "Cool white".to_string());
//     res.insert(14, "Night light".to_string());
//     res.insert(15, "Focus".to_string());
//     res.insert(16, "Relax".to_string());
//     res.insert(17, "True colors".to_string());
//     res.insert(18, "TV time".to_string());
//     res.insert(19, "Plantgrowth".to_string());
//     res.insert(20, "Spring".to_string());
//     res.insert(21, "Summer".to_string());
//     res.insert(22, "Fall".to_string());
//     res.insert(23, "Deepdive".to_string());
//     res.insert(24, "Jungle".to_string());
//     res.insert(25, "Mojito".to_string());
//     res.insert(26, "Club".to_string());
//     res.insert(27, "Christmas".to_string());
//     res.insert(28, "Halloween".to_string());
//     res.insert(29, "Candlelight".to_string());
//     30: "Golden white",
//     res.insert(31, "Pulse".to_string());
//     res.insert(32, "Steampunk".to_string());
//     res.insert(1000, "Rhythm".to_string());
// })
pub const SCENES: Lazy<HashMap<u32, String>> = Lazy::new(|| {
    map! {
        1 => "Ocean".to_string(),
        2 => "Romance".to_string(),
        3 => "Sunset".to_string(),
        4 => "Party".to_string(),
        5 => "Fireplace".to_string(),
        6 => "Cozy".to_string(),
        7 => "Forest".to_string(),
        8 => "Pastel Colors".to_string(),
        9 => "Wake up".to_string(),
        10 => "Bedtime".to_string(),
        11 => "Warm White".to_string(),
        12 => "Daylight".to_string(),
        13 => "Cool white".to_string(),
        14 => "Night light".to_string(),
        15 => "Focus".to_string(),
        16 => "Relax".to_string(),
        17 => "True colors".to_string(),
        18 => "TV time".to_string(),
        19 => "Plantgrowth".to_string(),
        20 => "Spring".to_string(),
        21 => "Summer".to_string(),
        22 => "Fall".to_string(),
        23 => "Deepdive".to_string(),
        24 => "Jungle".to_string(),
        25 => "Mojito".to_string(),
        26 => "Club".to_string(),
        27 => "Christmas".to_string(),
        28 => "Halloween".to_string(),
        29 => "Candlelight".to_string(),
        30 => "Golden white".to_string(),
        31 => "Pulse".to_string(),
        32 => "Steampunk".to_string(),
        1000 => "Rhythm".to_string(),
    }
});
