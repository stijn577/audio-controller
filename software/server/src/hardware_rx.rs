use anyhow::Result;
use shared_data::{AppMsg, Message};

pub(crate) fn receive_message() -> Result<Message> {
    // TODO: read from usb and parse data
    Ok(Message::App(AppMsg::Slot0))
}
