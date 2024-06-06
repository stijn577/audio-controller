use serde::{Deserialize, Serialize};
use crate::N_SLIDERS;

#[derive(Serialize, Deserialize, Debug)]
pub struct AudioLevels {
    // array of AUDIO levels, (range 0-1024)
    data: [u16; N_SLIDERS],
}

impl AudioLevels {
    pub fn new(data: [u16; N_SLIDERS]) -> Self {
        Self { data }
    }
}
