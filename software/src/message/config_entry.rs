#[cfg(feature = "std")]
use crate::{message::error::MessageError, _SHELL, _SHELL_EXEC};
use serde::{Deserialize, Serialize};

#[cfg_attr(not(feature = "std"), derive(defmt::Format))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConfigEntry {
    Command(u8, alloc::vec::Vec<alloc::string::String>),
    KeyPress(u8, alloc::vec::Vec<u8>),
}

impl Default for ConfigEntry {
    fn default() -> Self {
        Self::Command(0, alloc::vec::Vec::new())
    }
}

#[cfg(feature = "std")]
impl ConfigEntry {
    pub(crate) async fn execute_entry(&self) -> Result<(), MessageError> {
        match self {
            ConfigEntry::Command(_, args) => self.execute_command(args).await,
            ConfigEntry::KeyPress(_, keycodes) => self.execute_keystrokes(keycodes).await,
        }
    }

    async fn execute_command(&self, args: &[alloc::string::String]) -> Result<(), MessageError> {
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

    async fn execute_keystrokes(&self, keycodes: &[u8]) -> Result<(), MessageError> {
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
