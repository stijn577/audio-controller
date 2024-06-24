use alloc::string::String;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SliderConfig<const N: usize>(heapless::Vec<Slider, N>);

impl<const N: usize> SliderConfig<N> {
    pub const fn new(config: heapless::Vec<Slider, N>) -> Self {
        Self(config)
    }
}

impl<const N: usize> Default for SliderConfig<N> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Slider {
    host_process: String,
    // bitmap: String,
}

impl Slider {
    pub fn new(host_process: String) -> Self {
        Self { host_process }
    }
}
