use anyhow::Result;
use shared_data::{AppLaunch, Message};

pub(crate) fn receive_message() -> Result<Message> {
    // TODO: read from usb and parse data
    Ok(Message::AppLaunch(AppLaunch::Spotify))
}
