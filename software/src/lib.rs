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

macros::buttons!(
    // define the number of slots on the hardware device => becomes const N_SLOTS: usize = ...;
    5,
    // define comands one by one, if more than number of slots are defined, macro panicks
    // discord command
    Command::new(SHELL)
        .arg(SHELL_EXEC)
        .args([
            "C:/Users/Stijn_Admin/AppData/Local/Discord/Update.exe",
            "--processStart",
            "Discord.exe"
        ])
        .status()
        .expect("Failed to launch discord!");
    Ok(()),
    // spotify command
    Command::new(SHELL)
        .arg(SHELL_EXEC)
        .arg("spotify.exe")
        .status()
        .expect("Failed to launch spotify!");
    Ok(()),
    // firefox command
    Command::new(SHELL)
        .arg(SHELL_EXEC)
        .arg("firefox.exe")
        .status()
        .expect("Failed to launch firefox!");
    Ok(()),
    // if there are more slots than commands, there is a Error ed when the launch function is called
    Err(ButtonError::NoMatchingCommand)
);

const N_SLIDERS: usize = 5;

#[derive(Serialize, Deserialize, Debug)]
pub struct SliderValues {
    // array of AUDIO levels, (range 0-1024)
    data: [u16; N_SLIDERS],
}

impl SliderValues {
    pub fn new(data: [u16; N_SLIDERS]) -> Self {
        Self { data }
    }
}

pub enum Message {
    Button(Button),
    SliderReport(SliderValues),
}
