#![cfg_attr(not(feature = "std"), no_std)]

// define launcher for shell execution (otherwise it might hang, so use shell closest to OS)
use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
const SHELL: &str = "powershell";
#[cfg(target_os = "windows")]
const SHELL_EXEC: &str = "-Command";

#[cfg(target_os = "linux")]
const SHELL: &str = "/bin/sh";
#[cfg(target_os = "linux")]
const SHELL_EXEC: &str = "-c";

const N_SLIDERS: usize = 5;

macros::message!(14);

#[derive(Serialize, Deserialize, Debug)]
pub struct AudioLevels {
    // array of AUDIO levels, (range 0-1024)
    data: [u16; N_SLIDERS],
}

impl AudioLevels {
    pub fn new(data: [u16; N_SLIDERS]) -> Self {
        Self { data }
    }
}

#[cfg(feature = "std")]
#[derive(thiserror::Error, Debug)]
pub enum MessageParseError {
    ReadFileError(#[from] std::io::Error),
    NoMatchingCommand,
    ExitFailure,
}

#[cfg(feature = "std")]
impl std::fmt::Display for MessageParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl Message {
    pub async fn process_message(&self) -> Result<(), MessageParseError> {
        match self {
            Self::AudioLevels(_) => todo!("Implement levels parsing logic"),
            _ => self.process_slot().await,
        }
    }

    async fn process_slot(&self) -> Result<(), MessageParseError> {
        let config = tokio::fs::read_to_string("../config.csv")
            .await
            .map_err(|err| MessageParseError::ReadFileError(err))?;

        let mut config_lines = config.lines();

        let out = config_lines
            .nth(self.as_usize())
            .ok_or(MessageParseError::NoMatchingCommand)?;

        let mut line_iter = out.split(';');

        match line_iter.next() {
            Some(c) => {
                if c == "C" {
                    let args = line_iter.collect::<Vec<_>>();
                    self.command(&args).await?
                }
            }
            None => todo!(),
        }

        Ok(())
    }

    fn as_usize(&self) -> usize {
        match self {
            Message::Slot0 => 0,
            Message::Slot1 => 1,
            Message::Slot2 => 2,
            Message::Slot3 => 3,
            Message::Slot4 => 4,
            Message::Slot5 => 5,
            Message::Slot6 => 6,
            Message::Slot7 => 7,
            Message::Slot8 => 8,
            Message::Slot9 => 9,
            Message::Slot10 => 10,
            Message::Slot11 => 11,
            Message::Slot12 => 12,
            Message::Slot13 => 13,
            Message::AudioLevels(_) => unreachable!(),
        }
    }

    async fn command(&self, args: &[&str]) -> Result<(), MessageParseError> {
        println!("{:?}", args);
        let status = std::process::Command::new(SHELL)
            .arg(SHELL_EXEC)
            .args(args)
            .status()?;
        match status.success() {
            true => Ok(()),
            false => Err(MessageParseError::ExitFailure),
        }
    }
}
