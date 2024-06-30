use crate::action::Action;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BtnConfig<const N: usize>(heapless::Vec<Action, N>);

// impl<const N: usize> Default for BtnConfig<N> {
//     fn default() -> Self {
//         Self(Default::default())
//     }
// }

unsafe impl<const N: usize> Sync for BtnConfig<N> {}

impl<const N: usize> BtnConfig<N> {
    pub const fn new() -> Self {
        Self(heapless::Vec::new())
    }
}
