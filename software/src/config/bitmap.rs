use crate::N_BITMAPS;
use alloc::{string::String, vec::Vec};
use heapless as hl;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
pub struct BMPConfig(hl::LinearMap<String, RawBmpData, N_BITMAPS>);

impl BMPConfig {
    pub fn new(config: hl::LinearMap<String, RawBmpData, N_BITMAPS>) -> Self {
        Self(config)
    }
}

impl Iterator for BMPConfig {
    type Item = (String, RawBmpData);

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .into_iter()
            .next()
            .map(|(k, v)| (k.clone(), v.clone()))
    }
}

#[cfg(feature="defmt")]
impl defmt::Format for BMPConfig {
    fn format(&self, fmt: defmt::Formatter) {
        self.0.iter().for_each(|(k, v)| {
            defmt::write!(fmt, "BMPConfig({:?}, {:?})", k, v.raw_data.len());
        })
    }
}

#[cfg_attr(feature="defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Hash)]
pub struct RawBmpData {
    w: usize,
    h: usize,
    raw_data: Vec<u8>,
}
