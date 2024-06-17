#![feature(never_type)]
use anyhow::Context;
use log::info;
use log::warn;
use shared_data::{action::Action, message::Message};
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;
use tokio::time::Duration;
use tokio_serial::SerialPort;
use tokio_serial::SerialPortBuilderExt;

mod hardware_rx;
mod os_commands;

#[tokio::main]
async fn main() -> anyhow::Result<!> {
    env_logger::init();

    let port = tokio_serial::available_ports()
        .context("Could not list ports!")?
        .into_iter()
        .map(|dev| {
            info!("Found port: {:#?}", dev);
            dev
        });

    let mut usb = tokio_serial::new("COM19", 115200)
        .baud_rate(115200)
        .data_bits(tokio_serial::DataBits::Eight)
        .flow_control(tokio_serial::FlowControl::None)
        .parity(tokio_serial::Parity::None)
        .timeout(Duration::from_millis(1000))
        .open_native_async()
        .with_context(|| "Failed to open serial port")?;

    info!("Usb made");

    let msg = Message::Action(Action::Command(vec![String::from("firefox.exe")]));
    let msg_cbor = msg.serialize().context("Failed to serialize")?;
    info!("Message ready!");

    Ok(loop {
        sleep(Duration::from_millis(1000)).await;

        // wait for USB to be available to write
        if (usb.writable().await).is_ok() {
            if let Ok(n) = usb.try_write(&msg_cbor) {
                info!("Message sent!");
            } else {
                warn!("Failed to write to serial port");
            }
        }

        let mut buf = [0u8; 1024];
        // usb.flush();
        // info!("usb flushed, waiting for message!");

        sleep(Duration::from_millis(1000)).await;

        // wait for the USB to be available to read
        if (usb.readable().await).is_ok() {
            if let Ok(n) = usb.try_read(&mut buf) {
                info!("Message received: {:?}", Message::deserialize(&buf));
            } else {
                warn!("Failed to read from serial port");
            }
        }
    })

    // if cfg!(target_os = "windows") {
    // TODO: receive slot messages from controller, instead of hardcoding here
    //
    // _audio_control().await;
    //
    // let msg = Message::ConfigEntry(ConfigEntry::Command(0, vec!["spotify.exe".to_string()]));
    // let out = msg.serialize().context("Failed to serialize")?;
    // let out = Message::deserialize(&out).context("Failed to deserialize")?;
    // info!("{:?}", out);
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
