use alloc::{string::String, vec};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SliderConfig<const N: usize>(heapless::Vec<Slider, N>);

impl<const N: usize> SliderConfig<N> {
    pub const fn new() -> Self {
        Self(heapless::Vec::new())
    }
}

unsafe impl<const N: usize> Sync for SliderConfig<N> {}

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
    pub const fn new(host_process: String) -> Self {
        Self { host_process }
    }
}
