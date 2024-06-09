use alloc::{boxed::Box, string::String, vec::Vec};
use serde::{Deserialize, Serialize};

use crate::{action::Action, message::Message};

pub mod bitmap;
pub mod hardware;
pub mod slider;
pub mod software;

#[cfg_attr(not(feature = "std"), derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BtnEntry {
    exec: Action,
    bitmap: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) hw_btn_cfg: crate::config::hardware::HWBtnConfig,
    pub(crate) sw_btn_cfg: crate::config::software::SWBtnConfig,
    pub(crate) slider_cfg: crate::config::slider::SliderConfig,
    pub(crate) bitmap_cfg: crate::config::bitmap::BMPConfig,
}

impl Config {
    pub async fn from_config(&mut self) -> Vec<Message> {
        let mut vec = Vec::with_capacity(4);

        vec[0] = Message::SWBtnConfig(Box::new(self.sw_btn_cfg.clone()));
        vec[1] = Message::HWBtnConfig(Box::new(self.hw_btn_cfg.clone()));
        vec[2] = Message::SliderConfig(Box::new(self.slider_cfg.clone()));

        for (name, data) in self.bitmap_cfg.clone() {
            vec.push(Message::BitMap(name, data));
        }

        vec
    }
}

// #[cfg_attr(not(feature = "std"), derive(defmt::Format))]
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct BaseConfig {
//     h_btn_cfg: crate::config::hardware_config::HWBtnConfig,
//     s_btn_cfg: crate::config::software_config::SWBtnConfig,
//     slider_cfg: crate::config::slider_config::SliderConfig,
// }

// impl BaseConfig {
//     pub fn new(
//         h_btn_cfg: crate::config::hardware_config::HWBtnConfig,
//         s_btn_cfg: crate::config::software_config::SWBtnConfig,
//         slider_cfg: crate::config::slider_config::SliderConfig,
//     ) -> Self {
//         Self {
//             h_btn_cfg,
//             s_btn_cfg,
//             slider_cfg,
//         }
//     }
// }