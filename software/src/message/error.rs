use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use thiserror::Error;
#[cfg(not(feature = "std"))]
use thiserror_no_std::Error;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Error)]
pub enum MessageError {
    NoMatchingCommand,
    CommandLaunch,
    CommandExitFailure,
    NoMatchingKey,
    USBWriteFailure,
    Cbor,
}

#[cfg(feature = "std")]
impl std::fmt::Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
