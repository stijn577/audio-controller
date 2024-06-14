use crate::config::BtnEntry;
use crate::{N_SCREENS, N_SWB_PER_SCREEN};
use heapless as hl;
use serde::{Deserialize, Serialize};

/// A struct representing the configuration for a set of switches.
///
/// This struct contains a 2D vector of `BtnEntry` instances, where the first dimension represents the number of screens,
/// and the second dimension represents the number of buttons per screen.
///
/// # Examples
///
/// ```
/// let config = SWBtnConfig::new(vec![
///     vec![BtnEntry::new(1, "firefox"), BtnEntry::new(2, "spotify")],
///     vec![BtnEntry::new(3, "discord"), BtnEntry::new(4, "Switch 4")]
/// ]);
/// ```
///
/// # Fields
///
/// * `0` - A 2D vector of `BtnEntry` instances.
#[cfg_attr(feature="defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SWBtnConfig(hl::Vec<hl::Vec<BtnEntry, N_SWB_PER_SCREEN>, N_SCREENS>);

impl SWBtnConfig {
    /// Creates a new instance of `SWBtnConfig` with the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - A 2D vector of `BtnEntry` instances, where the first dimension represents the number of screens,
    ///   and the second dimension represents the number of buttons per screen.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = SWBtnConfig::new(vec![
    ///     vec![BtnEntry::new(1, Some("firefox")), BtnEntry::new(2, Some("spotify"))],
    ///     vec![BtnEntry::new(3, Some("discord")), BtnEntry::new(4, None)]
    /// ]);
    /// ```
    ///
    /// # Returns
    ///
    /// A new instance of `SWBtnConfig` with the provided configuration.
    pub const fn new(config: hl::Vec<hl::Vec<BtnEntry, N_SWB_PER_SCREEN>, N_SCREENS>) -> Self {
        Self(config)
    }
    /// Updates the entire configuration of the `SWBtnConfig` instance with the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to a `SWBtnConfig` instance containing the new configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut config = SWBtnConfig::new(vec![
    ///     vec![BtnEntry::new(1, Some("firefox")), BtnEntry::new(2, Some("spotify"))],
    ///     vec![BtnEntry::new(3, Some("discord")), BtnEntry::new(4, None)]
    /// ]);
    /// let new_config = SWBtnConfig::new(vec![
    ///     vec![BtnEntry::new(1, Some("chrome")), BtnEntry::new(2, Some("telegram"))],
    ///     vec![BtnEntry::new(3, Some("zoom")), BtnEntry::new(4, Some("chrome"))]
    /// ]);
    /// config.set_to(new_config);
    /// ```
    ///
    /// # Returns
    ///
    /// This function does not return any value. It modifies the `SWBtnConfig` instance in-place.
    pub fn set_to(&mut self, config: Self) {
        self.0 = config.0;
    }
    /// Updates a single entry in the configuration of the `SWBtnConfig` instance.
    ///
    /// # Arguments
    ///
    /// * `screen` - The index of the screen where the button is located.
    /// * `button` - The index of the button within the specified screen.
    /// * `new_values` - The new values for the button entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::config::SWBtnConfig;
    /// let mut config = SWBtnConfig::new(vec![
    ///     vec![BtnEntry::new(1, "firefox"), BtnEntry::new(2, "spotify")],
    ///     vec![BtnEntry::new(3, "discord"), BtnEntry::new(4, None)]
    /// ]);
    /// config.set_one_to(1, 2, BtnEntry::new(1, "chrome"));
    /// ```
    pub fn set_one_to(&mut self, screen: usize, button: usize, new_values: BtnEntry) {
        if screen < N_SCREENS && button < N_SWB_PER_SCREEN {
            self.0[screen][button] = new_values;
        }
    }
}
