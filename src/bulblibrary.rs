use crate::{Result, WizError};
use buildstructor::buildstructor;

use rayon::prelude::*;

#[derive(Debug, Default)]
pub struct Features {
    pub color: bool,
    pub color_tmp: bool,
    pub effect: bool,
    pub brightness: bool,
    pub dual_head: bool,
    pub name: String,
    pub kelvin_range: Option<KelvinRange>,
    pub fw_version: Option<String>,
    pub white_channels: Option<i64>,
    pub white_to_color_ratio: Option<i64>,
}

#[buildstructor]
impl Features {
    #[builder]
    pub fn rgb_new(
        name: String,
        fw_version: Option<String>,
        effect: bool,
        dual_head: bool,
        white_channels: Option<i64>,
        white_to_color_ratio: Option<i64>,
        kelvin_range: Option<KelvinRange>,
    ) -> Self {
        Self {
            color: true,
            color_tmp: true,
            brightness: true,
            name,
            fw_version,
            effect,
            dual_head,
            white_channels,
            white_to_color_ratio,
            kelvin_range,
        }
    }
    #[builder]
    pub fn tw_new(
        name: String,
        fw_version: Option<String>,
        effect: bool,
        dual_head: bool,
        white_channels: Option<i64>,
        white_to_color_ratio: Option<i64>,
        kelvin_range: Option<KelvinRange>,
    ) -> Self {
        Self {
            color: false,
            color_tmp: true,
            brightness: true,
            name,
            fw_version,
            effect,
            dual_head,
            white_channels,
            white_to_color_ratio,
            kelvin_range,
        }
    }
    #[builder]
    pub fn dw_new(
        name: String,
        fw_version: Option<String>,
        effect: bool,
        dual_head: bool,
        white_channels: Option<i64>,
        white_to_color_ratio: Option<i64>,
        kelvin_range: Option<KelvinRange>,
    ) -> Self {
        Self {
            color: false,
            color_tmp: false,
            brightness: true,
            name,
            fw_version,
            effect,
            dual_head,
            white_channels,
            white_to_color_ratio,
            kelvin_range,
        }
    }
    #[builder]
    pub fn sock_new(
        name: String,
        fw_version: Option<String>,
        effect: bool,
        dual_head: bool,
        white_channels: Option<i64>,
        white_to_color_ratio: Option<i64>,
        kelvin_range: Option<KelvinRange>,
    ) -> Self {
        Self {
            color: false,
            color_tmp: false,
            brightness: false,
            name,
            fw_version,
            effect,
            dual_head,
            white_channels,
            white_to_color_ratio,
            kelvin_range,
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
    Rgb(Features),
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
            .ok_or(WizError::NoIdent(module_name.to_string()))?;
        let dual = ident.contains("DH");
        let k_range = if let Some(mut k_list) = kelvin_list {
            k_list.par_sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
            let min = k_list.last().ok_or(WizError::NoMinimum)?;
            let max = k_list.first().ok_or(WizError::NoMaximum)?;
            Some(KelvinRange::new(*max, *min))
        } else {
            None
        };
        let bulb_type = {
            if ident.contains("RGB") {
                let feat = Features::rgb_builder()
                    .dual_head(dual)
                    .effect(true)
                    .and_fw_version(fw_version)
                    .and_white_channels(white_channels)
                    .and_white_to_color_ratio(white_to_color_ratio)
                    .and_kelvin_range(k_range)
                    .name(module_name)
                    .build();
                BulbClass::Rgb(feat)
            } else if ident.contains("TW") {
                let feat = Features::tw_builder()
                    .dual_head(dual)
                    .effect(true)
                    .and_fw_version(fw_version)
                    .and_white_channels(white_channels)
                    .and_white_to_color_ratio(white_to_color_ratio)
                    .and_kelvin_range(k_range)
                    .name(module_name.to_string())
                    .build();
                BulbClass::TW(feat)
            } else if ident.contains("SOCKET") {
                let feat = Features::sock_builder()
                    .dual_head(dual)
                    .effect(false)
                    .and_fw_version(fw_version)
                    .and_white_channels(white_channels)
                    .and_white_to_color_ratio(white_to_color_ratio)
                    .and_kelvin_range(k_range)
                    .name(module_name.to_string())
                    .build();
                BulbClass::Socket(feat)
            } else {
                let eff = ident.contains("DH") || ident.contains("SH");
                let feat = Features::dw_builder()
                    .dual_head(dual)
                    .effect(eff)
                    .and_fw_version(fw_version)
                    .and_white_channels(white_channels)
                    .and_white_to_color_ratio(white_to_color_ratio)
                    .and_kelvin_range(k_range)
                    .name(module_name.to_string())
                    .build();
                BulbClass::DW(feat)
            }
        };
        Ok(bulb_type)
    }
}
