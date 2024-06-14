use crate::action::Action;
use crate::N_HWB;
use heapless as hl;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature="defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HWBtnConfig(hl::Vec<Action, N_HWB>);

impl HWBtnConfig {
    pub const fn new(config: hl::Vec<Action, N_HWB>) -> Self {
        Self(config)
    }

    pub fn set_to(&mut self, config: Self) {
        self.0 = config.0;
    }
}
