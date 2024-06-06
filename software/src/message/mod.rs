use serde::{Deserialize, Serialize};
use config_entry::ConfigEntry;
use error::MessageError;

pub mod config_entry;
pub mod error;
pub mod audiolevels;

#[cfg_attr(not(feature = "std"), derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    ConfigEntry(ConfigEntry),
}

impl Message {
    pub fn serialize(&self) -> Result<alloc::vec::Vec<u8>, MessageError> {
        serde_cbor::to_vec(&self).map_err(|_| MessageError::Cbor)
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, MessageError> {
        serde_cbor::from_slice(data).map_err(|_| MessageError::Cbor)
    }
}

#[cfg(feature = "std")]
impl Message {
    pub async fn execute_entry(&self) -> Result<(), MessageError> {
        match self {
            Message::ConfigEntry(entry) => entry.execute_entry().await,
        }
    }
}



