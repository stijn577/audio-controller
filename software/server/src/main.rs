use anyhow::Context;
use shared_data::message::{config_entry::ConfigEntry, Message};

mod hardware_rx;
mod os_commands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if cfg!(target_os = "windows") {
        // TODO: receive slot messages from controller, instead of hardcoding here

        let msg = Message::ConfigEntry(ConfigEntry::Command(0, vec!["spotify.exe".to_string()]));
        let out = msg.serialize().context("Failed to serialize")?;
        let out = Message::deserialize(&out).context("Failed to deserialize")?;
        println!("{:?}", out);
        out.execute_entry().await.context("Failed to launch application")?;

        let msg = Message::ConfigEntry(ConfigEntry::Command(0, vec!["firefox.exe".to_string()]));
        let out = msg.serialize().context("Failed to serialize")?;
        let out = Message::deserialize(&out).context("Failed to deserialize")?;
        println!("{:?}", out);

        out.execute_entry().await.context("Failed to launch application")?;

        // let thread0 = tokio::spawn(process(Message::));
        // let thread1 = tokio::spawn(process(Message::));
        // let thread2 = tokio::spawn(process(Message::));
        // let thread3 = tokio::spawn(process(Message::));
        // let thread4 = tokio::spawn(process(Message::));
        // let thread5 = tokio::spawn(process(Message::));
        // let _ = join!(thread0, thread1, thread2, thread3, thread4, thread5);

        // let x = tokio::spawn(keypress(Button::Slot3));
        // let thread6c = tokio::spawn(handle_message(Message::AudioLevels());

        // let launch_discord_fut = tokio::spawn(handle_message(Message::App(App::Slot0)));
        // let launch_spotify_fut = tokio::spawn(handle_message(Message::App(App::Slot1)));
        // let launch_firefox_fut = tokio::spawn(handle_message(Message::App(App::Slot2)));
    } else {
        todo!("Linux implementation here")
    }

    Ok(())
}

