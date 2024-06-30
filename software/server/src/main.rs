#![feature(never_type)]

use anyhow::Context;
use log::{info, warn};
use shared_data::action::Action;
use shared_data::message::Message;
use tokio::join;
use tokio::time::sleep;
use tokio::time::Duration;
use tokio_serial::SerialPortBuilderExt;

mod hardware_rx;
mod os_commands;

#[tokio::main]

async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let com = std::env::args().last().unwrap();
    let com = com.trim();

    info!("Serial port: {}", com);

    let usb_fut = usb_task(com);
    let bleh_fut = bleh_task();

    let _ = join!(usb_fut, bleh_fut);

    Ok(())
}

pub async fn bleh_task() {}

pub async fn usb_task(com: &str) -> anyhow::Result<()> {
    let tep = tokio_serial::available_ports().context("Could not list ports!")?.into_iter().map(|dev| {
        info!("Found port: {:#?}", dev);
        dev
    });

    let msg = Message::BtnPress(Action::default());
    let msg_cbor = msg.serialize().context("Failed to serialize")?;
    info!("{:?}", msg_cbor);


    info!("Message ready!");

    let usb_cfg = tokio_serial::new(com, 115200)
        .baud_rate(115200)
        .data_bits(tokio_serial::DataBits::Eight)
        .flow_control(tokio_serial::FlowControl::None)
        .parity(tokio_serial::Parity::None)
        .timeout(Duration::from_millis(1000));

    loop {
        if let Ok(mut usb) = usb_cfg.clone().open_native_async() {
            // wait for USB to be available to write
            if (usb.writable().await).is_ok() {
                if let Ok(_n) = usb.try_write(&msg_cbor) {
                    info!("Message sent!");
                } else {
                    warn!("Failed to write to serial port");
                }
            }

            let mut buf = [0u8; 4096];

            let mut msg: Option<Message> = None;
            // usb.flush();
            // info!("usb flushed, waiting for message!");

            //wait for the USB to be available to read
            if (usb.readable().await).is_ok() {
                if let Ok(n) = usb.try_read(&mut buf) {
                    msg = Some(Message::deserialize(&buf[0..n])?);
                    info!("Message received: {:?}", msg);
                } else {
                    warn!("Failed to read from serial port");
                }
            }

            if let Some(Message::BtnPress(action)) = msg {
                if let Ok(_) = action.perform().await {
                    info!("Action performed!");
                } else {
                    warn!("Action failed!");
                }
            }
        } else {
            warn!("Failed to open serial port");
        }
        sleep(Duration::from_secs(10)).await;
    }

    Ok(())
}
