use color_eyre::eyre::{eyre, ContextCompat};
use color_eyre::Result;
use rayon::prelude::*;

#[derive(Debug, Default)]
pub struct Features {
    pub color: bool,
    pub color_tmp: bool,
    pub effect: bool,
    pub brightness: bool,
    pub dual_head: bool,
    pub name: Option<String>,
    pub kelvin_range: Option<KelvinRange>,
    pub fw_version: Option<String>,
    pub white_channels: Option<i64>,
    pub white_to_color_ratio: Option<i64>,
}

impl Features {
    pub fn from_data(
        module_name: &str,
        kelvin_list: Option<Vec<f64>>,
        fw_version: Option<String>,
        white_channels: Option<i64>,
        white_to_color_ratio: Option<i64>,
    ) -> Result<Self> {
        let ident = module_name
            .split('_')
            .nth(1)
            .ok_or(eyre!("No identifier in {}", module_name))?;
        let (bulb_type, effect) = {
            if ident.contains("RGB") {
                (BulbClass::RGB, true)
            } else if ident.contains("TW") {
                (BulbClass::TW, true)
            } else if ident.contains("SOCKET") {
                (BulbClass::Socket, false)
            } else {
                let eff = ident.contains("DH") || ident.contains("SH");
                (BulbClass::DW, eff)
            }
        };
        let dual = ident.contains("DH");
        let k_range = if let Some(mut k_list) = kelvin_list {
            k_list.par_sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
            let min = k_list.last().wrap_err("No minimum value")?;
            let max = k_list.first().wrap_err("No maximum value")?;
            Some(KelvinRange::new(*max, *min))
        } else {
            None
        };
        let mut feat: Features = bulb_type.into();
        feat.dual_head = dual;
        feat.effect = effect;
        feat.fw_version = fw_version;
        feat.white_channels = white_channels;
        feat.white_to_color_ratio = white_to_color_ratio;
        Ok(feat)
    }
    pub fn fill_rgb(mut self) -> Self {
        self.color = true;
        self.color_tmp = true;
        self.brightness = true;
        self
    }
    pub fn fill_tw(mut self) -> Self {
        self.brightness = true;
        self.color = false;
        self.color_tmp = true;
        self
    }
    pub fn fill_dw(mut self) -> Self {
        self.brightness = true;
        self.color = false;
        self.color_tmp = false;
        self
    }
    pub fn fill_sock(mut self) -> Self {
        self.brightness = false;
        self.color = false;
        self.color_tmp = false;
        self
    }
    pub fn set_rest(
        mut self,
        dual_head: bool,
        effect: bool,
        fw_version: Option<String>,
        white_channels: Option<i64>,
        white_to_color_ratio: Option<i64>,
    ) -> Self {
        self.dual_head = dual_head;
        self.effect = effect;
        self.fw_version = fw_version;
        self.white_channels = white_channels;
        self.white_to_color_ratio = white_to_color_ratio;
        self
    }
    pub fn new(
        color: bool,
        color_tmp: bool,
        effect: bool,
        brightness: bool,
        dual_head: bool,
        name: Option<String>,
        kelvin_range: Option<KelvinRange>,
        fw_version: Option<String>,
        white_channels: Option<i64>,
        white_to_color_ratio: Option<i64>,
    ) -> Self {
        Self {
            color,
            color_tmp,
            effect,
            brightness,
            dual_head,
            name,
            kelvin_range,
            fw_version,
            white_channels,
            white_to_color_ratio,
        }
    }
}

#[derive(Debug, Default)]
pub struct KelvinRange {
    max: f64,
    min: f64,
}

impl KelvinRange {
    pub fn new(max: f64, min: f64) -> Self {
        Self { max, min }
    }
}

pub enum BulbClass {
    /// Tunable White
    ///
    /// Have Cool White and Warm White LEDs.
    TW(Features),
    /// Dimmable White
    ///
    /// Have only Dimmable white LEDs.
    DW(Features),
    /// RGB Tunable
    ///
    /// Have RGB LEDs.
    RGB(Features),
    /// Socket
    ///
    /// Smart socket with only on/off.
    Socket(Features),
}

impl BulbClass {
    pub fn from_data(
        module_name: &str,
        kelvin_list: Option<Vec<f64>>,
        fw_version: Option<String>,
        white_channels: Option<i64>,
        white_to_color_ratio: Option<i64>,
    ) -> Result<Self> {
        let ident = module_name
            .split('_')
            .nth(1)
            .ok_or(eyre!("No identifier in {}", module_name))?;
        let dual = ident.contains("DH");
        let k_range = if let Some(mut k_list) = kelvin_list {
            k_list.par_sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
            let min = k_list.last().wrap_err("No minimum value")?;
            let max = k_list.first().wrap_err("No maximum value")?;
            Some(KelvinRange::new(*max, *min))
        } else {
            None
        };
        let bulb_type = {
            if ident.contains("RGB") {
                let feat = Features::default().fill_rgb().set_rest(
                    dual,
                    true,
                    fw_version,
                    white_channels,
                    white_to_color_ratio,
                );
                BulbClass::RGB(feat)
            } else if ident.contains("TW") {
                let feat = Features::default().fill_tw().set_rest(
                    dual,
                    true,
                    fw_version,
                    white_channels,
                    white_to_color_ratio,
                );
                BulbClass::TW(feat)
            } else if ident.contains("SOCKET") {
                let feat = Features::default().fill_sock().set_rest(
                    dual,
                    false,
                    fw_version,
                    white_channels,
                    white_to_color_ratio,
                );
                BulbClass::Socket(feat)
            } else {
                let eff = ident.contains("DH") || ident.contains("SH");
                let feat = Features::default().fill_dw().set_rest(
                    dual,
                    eff,
                    fw_version,
                    white_channels,
                    white_to_color_ratio,
                );
                BulbClass::DW(feat)
            }
        };
        Ok(bulb_type)
    }
}
