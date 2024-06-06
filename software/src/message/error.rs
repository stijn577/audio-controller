use serde::{Deserialize, Serialize};
#[cfg(feature="std")]
use thiserror;
#[cfg(not(feature="std"))]
use thiserror_no_std;


#[cfg_attr(feature = "std", derive(thiserror::Error))]
#[cfg_attr(not(feature = "std"), derive(thiserror_no_std::Error))]
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageError {
    NoMatchingCommand,
    CommandLaunch,
    CommandExitFailure,
    NoMatchingKey,
    Cbor,
}

#[cfg(feature = "std")]
impl std::fmt::Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
