use alloc::{string::String, vec::Vec};
use heapless as hl;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature="defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SliderConfig(hl::Vec<Slider, 5>);

impl SliderConfig {
    pub const fn new(config: hl::Vec<Slider, 5>) -> Self {
        Self(config)
    }
}

#[cfg_attr(feature="defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Slider {
    host_process_name: String,
    bitmap: String,
}
