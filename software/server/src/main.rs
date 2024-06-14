use std::{
    io::{Read, Write},
    time::Duration,
};

use anyhow::Context;
use os_commands::_audio_control;

use shared_data::{action::Action, message::Message};
use tokio::time::sleep;
use tokio_serial::SerialPortBuilderExt;

mod hardware_rx;
mod os_commands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut usb = tokio_serial::new("COM18", 115200)
        .baud_rate(115200)
        .data_bits(tokio_serial::DataBits::Eight)
        .flow_control(tokio_serial::FlowControl::None)
        .parity(tokio_serial::Parity::None)
        .timeout(Duration::from_millis(1000))
        .open_native_async()
        .with_context(|| "Failed to open serial port")?;

    println!("Usb made!");

    let msg = Message::Action(Action::Command(vec![String::from("firefox.exe")]));
    let msg_cbor = msg.serialize().context("Failed to serialize")?;
    println!("Message ready!");

    loop {
        sleep(Duration::from_millis(1000)).await;

        if (usb.writable().await).is_ok() {
            usb.write(&msg_cbor)
                .context("Failed to write to serial port")?;
            println!("Message sent!");
        }

        let mut buf = [0u8; 1024];
        if (usb.readable().await).is_ok() {
            let n = usb
                .read(&mut buf)
                .context("Failed to read from serial port")?;

            println!("Message received");
            println!("{:?}", String::from_utf8_lossy(&buf[0..n]));
        }
    }

    Ok(())

    // if cfg!(target_os = "windows") {
    // TODO: receive slot messages from controller, instead of hardcoding here
    //
    // _audio_control().await;
    //
    // let msg = Message::ConfigEntry(ConfigEntry::Command(0, vec!["spotify.exe".to_string()]));
    // let out = msg.serialize().context("Failed to serialize")?;
    // let out = Message::deserialize(&out).context("Failed to deserialize")?;
    // println!("{:?}", out);
    // out.execute_entry()
    //     .await
    // .context("Failed to launch application")?;
    //
    // let msg = Message::ConfigEntry(ConfigEntry::Command(0, vec!["firefox.exe".to_string()]));
    // let out = msg.serialize().context("Failed to serialize")?;
    // let out = Message::deserialize(&out).context("Failed to deserialize")?;
    // println!("{:?}", out);
    //
    // out.execute_entry()
    //     .await
    //     .context("Failed to launch application")?;
    //
    // let thread0 = tokio::spawn(process(Message::));
    // let thread1 = tokio::spawn(process(Message::));
    // let thread2 = tokio::spawn(process(Message::));
    // let thread3 = tokio::spawn(process(Message::));
    // let thread4 = tokio::spawn(process(Message::));
    // let thread5 = tokio::spawn(process(Message::));
    // let _ = join!(thread0, thread1, thread2, thread3, thread4, thread5);
    //
    // let x = tokio::spawn(keypress(Button::Slot3));
    // let thread6c = tokio::spawn(handle_message(Message::AudioLevels());
    //
    // let launch_discord_fut = tokio::spawn(handle_message(Message::App(App::Slot0)));
    // let launch_spotify_fut = tokio::spawn(handle_message(Message::App(App::Slot1)));
    // let launch_firefox_fut = tokio::spawn(handle_message(Message::App(App::Slot2)));
    // } else {
    // todo!("Linux implementation here")
    // }
}
