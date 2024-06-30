use core::fmt::Debug;

#[cfg(feature = "std")]
use crate::{message::error::MessageError, _SHELL, _SHELL_EXEC};
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    /// Vec allows for combo commands (launching multiple entries with one tap)
    Command(u8),
    KbdHid(Vec<u8>),
    MediaHid(u16),
}

impl Default for Action {
    fn default() -> Self {
        Self::Command(0x00)
    }
}

#[cfg(feature = "std")]

impl Action {
    pub async fn perform(self) -> Result<(), MessageError> {
        match self {
            Self::KbdHid(ref vec) => self.emulate_keystrokes(&vec).await?,
            Self::Command(idx) => self.execute_command(idx).await?,
            _ => unreachable!("MediaHid actions should be ran on the client, not on the server."),
        }

        Ok(())
    }

    async fn execute_command(&self, _idx: u8) -> Result<(), MessageError> {
        // TODO: change this to parse from config file
        let args = &[String::from("firefox.exe")];

        match std::process::Command::new(_SHELL)
            .arg(_SHELL_EXEC)
            .args(args)
            .status()
        {
            Ok(status) => match status.success() {
                true => Ok(()),
                false => Err(MessageError::CommandExitFailure),
            },
            Err(_) => Err(MessageError::CommandLaunch),
        }
    }

    async fn emulate_keystrokes(&self, keycodes: &[u8]) -> Result<(), MessageError> {
        let mut keys = alloc::vec::Vec::new();

        // first parse all keys to see if they are valid before doing anything
        for &keycode in keycodes {
            let key = match inputbot::get_keybd_key(keycode as char) {
                Some(key) => Ok(key),
                None => Err(MessageError::NoMatchingKey),
            }?;
            keys.push(key);
        }

        // only then we execute all keypresses
        keys.iter().for_each(|key| {
            key.press();
            key.release();
        });

        Ok(())
    }
}
