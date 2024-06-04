use shared_data::{AppLaunch, Message};
use std::{process::Command, time::Duration};

mod hardware_rx;
mod os_commands;

fn main() {
    if cfg!(target_os = "windows") {
        loop {
            handle_message(Message::AppLaunch(AppLaunch::Discord));
            std::thread::sleep(Duration::from_millis(1000));
        }
    } else {
        todo!("Linux implementation here")
    }
}

fn handle_message(data: Message) {
    match data {
        Message::AppLaunch(app) => match app {
            AppLaunch::Discord => {
                Command::new("C:/Users/Stijn_Admin/AppData/Local/Discord/Update.exe")
                    .args(["--processStart", "Discord.exe"])
                    .output()
                    .expect("Failed to launch application");
            }
            AppLaunch::Spotify => {
                Command::new("spotify")
                    .output()
                    .expect("Failed to launch application");
            }
            AppLaunch::Firefox => {
                Command::new("firefox")
                    .output()
                    .expect("Failed to launch application");
            }
        },
        Message::AudioLevels(_) => todo!(),
    }
}
