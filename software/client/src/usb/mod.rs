use alloc::string::String;
use alloc::vec::Vec;
use defmt::warn;
use defmt::{info, println};
use embassy_stm32::usb_otg::Driver;
use embassy_stm32::usb_otg::Instance;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::Receiver;
use embassy_sync::channel::Sender;
use embassy_time::Delay;
use embassy_time::Timer;
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::driver::EndpointError;
use serde::Serialize;
use shared_data::message::Message;

use crate::USB_PACKET_SIZE;

#[derive(thiserror_no_std::Error)]
pub(crate) struct Disconnected {
    #[source]
    pub(crate) _source: EndpointError,
}

impl From<EndpointError> for Disconnected {
    fn from(value: EndpointError) -> Self {
        match value {
            EndpointError::BufferOverflow => panic!("Buffer overflowed!"),
            EndpointError::Disabled => Disconnected { _source: value },
        }
    }
}

/// Asynchronously reads and writes messages over a USB connection.
///
/// This function continuously reads packets from the USB connection and writes them to the provided receiver.
/// It then reads messages from the provided receiver and writes them to the USB connection.
///
/// # Arguments
///
/// * `class` - A mutable reference to a CDC ACM class instance.
/// * `rx` - A receiver channel to receive messages.
/// * `tx` - A sender channel to send messages.
///
/// # Returns
///
/// A `Result` containing an error if the USB connection is disconnected, or `Ok(())` if the messages are successfully sent and received.
///
/// # Panics
///
/// Panics if the `EndpointError::BufferOverflow` is encountered.
///
/// # Examples
///
/// ```
/// use crate::usb_messaging;
///
/// #[tokio::main]
/// async fn main() {
///     // Initialize USB and CDC ACM class instances
///     let usb_instance = /* initialize USB instance */;
///     let cdc_acm_class = /* initialize CDC ACM class instance */;
///
///     // Create sender and receiver channels
///     let (tx, rx) = embassy_sync::channel::bounded(5);
///
///     // Start reading and writing messages
///     if let Ok(()) = usb_messaging(&mut cdc_acm_class, rx, tx).await {
///         println!("Messages sent and received successfully");
///     } else {
///         println!("Error sending and receiving messages");
///     }
/// }
/// ```
///
pub(crate) async fn usb_messaging<'d, T: Instance + 'd>(
    class: &mut CdcAcmClass<'d, Driver<'d, T>>,
    usb_transmit_channel: Receiver<'static, CriticalSectionRawMutex, Message, 5>,
    usb_received_channel: Sender<'static, CriticalSectionRawMutex, Message, 5>,
) -> Result<(), EndpointError> {
    let mut packet_buf = [0u8; USB_PACKET_SIZE];
    let mut msg_buf = Vec::with_capacity(USB_PACKET_SIZE);

    while let Ok(n) = class.read_packet(&mut packet_buf).await {
        msg_buf.push(packet_buf);
        packet_buf.fill(0);

        if n < USB_PACKET_SIZE {
            // break out once we see a packet is not MAX SIZE (this means the message is complete)
            break;
        }
    }

    let msg_buf = msg_buf.into_iter().flatten().collect::<Vec<_>>();


    if let Ok(msg) = Message::deserialize(&msg_buf) {
        println!("Received message: {:?}", msg);
        usb_received_channel.send(msg).await;
    } else {
        let msg = String::from_utf8_lossy(&msg_buf);
        if let Some(n) = msg.find('\0') {
            warn!(
                "Failed to deserialize message:\n\tData as String: {:?}",
                msg.split_at(n).0
            );
        }
    }

    if let Ok(msg) = usb_transmit_channel.try_receive() {
        for packet in msg.serialize().unwrap().chunks(USB_PACKET_SIZE) {
            class.write_packet(&packet).await.unwrap();
        }
    } else {
        println!("No message to send to server");
    }

    for chunk in msg_buf.chunks(64) {
        match class.write_packet(&chunk).await {
            Ok(_) => (),
            Err(_) => (),
        }
    }

    Ok(())
}

// pub(crate) async fn usb_read_message<'d, T: Instance + 'd>(
//     class: &mut CdcAcmClass<'d, Driver<'d, T>>,
// ) {
//     while let Ok(n) = class.read_packet(&mut buffer).await {
//         vec.push(buffer);
//     }
// }
