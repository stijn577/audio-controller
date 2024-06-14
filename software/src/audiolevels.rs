use crate::N_SLIDERS;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature="defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug)]
pub struct AudioLvls {
    // array of AUDIO levels, (range 0-1024)
    data: [u16; N_SLIDERS],
}

impl AudioLvls {
    pub fn new(data: [u16; N_SLIDERS]) -> Self {
        Self { data }
    }
}





