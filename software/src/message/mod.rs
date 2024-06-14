use crate::{
    action::Action,
    audiolevels::AudioLvls,
    config::{
        bitmap::RawBmpData, hardware::HWBtnConfig, slider::SliderConfig, software::SWBtnConfig,
        Config,
    },
};
use alloc::{boxed::Box, string::String, vec::Vec};
use error::MessageError;
use serde::{Deserialize, Serialize};

pub mod error;

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
    /// Request from the audio-controller to load the current configuration from PC host memory config files (static between sessions).
    RequestConfig,
    /// Reply from the audio-controller to confirm a message is delivered and handled.
    /// The bitmaps that are transmitted are very large, it's not possible to store them all in RAM on the audio-controller.
    /// For this reason, these replies are used to tell the PC that the controller is ready to receive the next message.
    SendNext,
    /// Might be unecessary
    Finished,
    /// Sends a software button configuration to/from the audio-controller.
    SWBtnConfig(Box<SWBtnConfig>),
    /// Sends a hardware button configuration to/from the audio-controller.
    HWBtnConfig(Box<HWBtnConfig>),
    /// Sends a slider configuration.
    SliderConfig(Box<SliderConfig>),
    /// Sends a bitmap image hash of the pixels, if these match with a hash on the controller, the new bitmap doesn't need to be sent
    /// Leaving a lot more CPU cycles for the controller to do other stuff.
    BitmapHash(String, String),
    /// Sends a bitmap image with the specified name and data.
    BitMap(String, RawBmpData),
    /// Sends an action message from the controller to the PC in preparation to be executed.
    Action(Action),
    /// Sends audio levels message from the controller to the PC. So the volumes of processes can be adjusted.
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
        serde_cbor::to_vec(&self).map_err(|_| MessageError::Cbor)
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
    pub fn deserialize(data: &[u8]) -> Result<Self, MessageError> {
        let data: Vec<u8> = data.iter().cloned().filter(|&b| b != 0).collect();
        serde_cbor::from_slice(&data).map_err(|_| MessageError::Cbor)
    }
}

#[cfg(not(feature = "std"))]
impl Message {
    pub async fn send(self) -> Result<Message, MessageError> {
        if let ser = self.serialize() {};
        todo!()
    }

    pub async fn send_messages(messages: Vec<Message>) -> Result<(), MessageError> {
        for msg in messages {
            msg.send().await?;
        }

        Ok(())
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
