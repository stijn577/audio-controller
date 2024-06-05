#![cfg_attr(not(feature = "std"), no_std)]

use serde::{Deserialize, Serialize};
#[cfg(target_os = "windows")]
const SHELL: &str = "powershell";
#[cfg(target_os = "windows")]
const SHELL_EXEC: &str = "-Command";

#[cfg(target_os = "linux")]
const SHELL: &str = "/bin/sh";
#[cfg(target_os = "linux")]
const SHELL_EXEC: &str = "-c";

macros::create_enum!(
    5, // define the number of slots on the hardware device
    // define comands one by one, if more than number of slots are defined, macro panicks
    Command::new(SHELL)
        .arg(SHELL_EXEC)
        .args([
            "C:/Users/Stijn_Admin/AppData/Local/Discord/Update.exe",
            "--processStart",
            "Discord.exe"
        ])
        .status()
        .expect("Failed to launch application!"),
    Command::new(SHELL)
        .arg(SHELL_EXEC)
        .arg("spotify")
        .status()
        .expect("Failed to launch application!"),
    Command::new(SHELL)
        .arg(SHELL_EXEC)
        .arg("-Command")
        .arg("firefox")
        .output()
        .expect("Failed to launch application!"),
    // if there are more slots than commands, there is a Error returned when the launch function is called (no semicolon, see macro implementation)
    return Err(AppMsgError::NoMatchingCommand)
);

#[derive(Serialize, Deserialize, Debug)]
pub struct AudioLevels {
    // array of AUDIO levels, (range 0-1024)
    data: [u16; 4],
}

pub enum Message {
    App(AppMsg),
    AudioLevels(AudioLevels),
}
