use crate::config::Config;
use crate::{action::Action, audiolevels::AudioLvls};
use crate::{prelude::*, SERIAL_PACKET_SIZE};
use alloc::vec::Vec;
use error::MessageError;
use serde::{Deserialize, Serialize};

pub mod error;

#[cfg(feature = "embassy")]
pub mod usb_nostd;
#[cfg(feature = "std")]
pub mod usb_std;

/// Enum representing different types of messages that can be sent or received.
///
/// Each variant of the enum represents a specific type of message, such as requesting configuration, sending the next message, or sending a bitmap image.
///
/// # Examples
///
/// ```
/// use crate::message::Message;
///
/// // Creating a new message of type `SWBtnConfig`
/// let sw_btn_config = SWBtnConfig { /* ... */ };
/// let sw_btn_config_message = Message::SWBtnConfig(Box::new(sw_btn_config));
/// ```
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    StartupConfiguration(Config),
    EditSWBtn(u8, Action),
    EditHWBtn(u8, Action),
    BtnPress(Action),
    AudioLvls(AudioLvls),
}

impl Message {
    /// Serializes the message into a vector of bytes.
    ///
    /// This method uses the `serde_cbor` crate to serialize the message into a vector of bytes. If the serialization fails, it returns a `Result` containing a `MessageError::Cbor` error.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::message::Message;
    ///
    /// // Create a new message of type `SWBtnConfig`
    /// let sw_btn_config = SWBtnConfig { /* ... */ };
    /// let sw_btn_config_message = Message::SWBtnConfig(Box::new(sw_btn_config));
    ///
    /// // Serialize the message
    /// let serialized_message: Result<Vec<u8>, MessageError> = sw_btn_config_message.serialize();
    /// ```
    ///
    /// # Errors
    ///
    /// This method returns a `Result` containing a `MessageError::Cbor` error if the serialization fails.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` containing a vector of bytes representing the serialized message.
    pub fn serialize(&self) -> Result<alloc::vec::Vec<u8>, MessageError> {
        cond_log!(debug!("serialize"));
        // let buf = Vec::new();
        let mut buf = Vec::new();
        ciborium::into_writer(self, &mut buf).map_err(|_| MessageError::Cbor)?;
        Ok(buf)
        // Ok(buf)
        // serde_cbor::to_vec(&self).map_err(|_| MessageError::Cbor)
    }

    /// Deserializes the message from a vector of bytes.
    ///
    /// This method uses the `serde_cbor` crate to deserialize the message from a vector of bytes. If the deserialization fails, it returns a `Result` containing a `MessageError::Cbor` error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use crate::message::Message;
    ///
    /// // Create a new message of type `SWBtnConfig`
    /// let sw_btn_config = SWBtnConfig { /* ... */ };
    /// let serialized_message: Vec<u8> = sw_btn_config.serialize().unwrap();
    ///
    /// // Deserialize the message
    /// let deserialized_message: Result<Message, MessageError> = Message::deserialize(&serialized_message);
    /// ```
    ///
    /// # Errors
    ///
    /// This method returns a `Result` containing a `MessageError::Cbor` error if the deserialization fails.
    ///
    /// # Returns
    ///
    /// This method returns a `Result` containing the deserialized message.
    pub fn deserialize(data: &[u8]) -> Result<Message, MessageError> {
        cond_log!(debug!("deserialize"));
        ciborium::from_reader(data).map_err(|_| MessageError::Cbor)
        // let data: Vec<u8> = data.iter().cloned().filter(|&b| b != 0).collect();
        // serde_cbor::from_slice(&data).map_err(|_| MessageError::Cbor)
    }
}

#[cfg(feature = "std")]
impl Message {
    pub async fn execute_entry(&self) -> Result<(), MessageError> {
        todo!()
    }

    pub async fn send_messages(messages: Vec<Message>) -> Result<(), MessageError> {
        for msg in messages {
            msg.send_to_client().await?;
        }

        Ok(())
    }

    pub async fn send_to_client(self) -> Result<(), MessageError> {
        todo!()
    }
}
