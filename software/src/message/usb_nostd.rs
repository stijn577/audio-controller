use alloc::vec::Vec;
use embassy_stm32::usb_otg::Instance;
use embassy_usb::{
    class::cdc_acm::{Receiver, Sender},
    driver::Driver,
};

use crate::SERIAL_PACKET_SIZE;

use super::{error::MessageError, Message};

trait MessageTransceiver: MessageSender + MessageReceiver {}

pub trait MessageSender {
    async fn send_message(&mut self, msg: Message) -> Result<(), MessageError>;
}

impl<'d, D> MessageSender for Sender<'d, D>
where
    D: Driver<'d>,
{
    async fn send_message(&mut self, msg: Message) -> Result<(), MessageError> {
        if let Ok(msg_cbor) = msg.serialize() {
            for chunk in msg_cbor.chunks(SERIAL_PACKET_SIZE.into()) {
                self.write_packet(chunk).await.map_err(|_| MessageError::USBWriteFailure)?;
            }
            self.write_packet(&[]).await.map_err(|_| MessageError::USBWriteFailure)?;

            Ok(())
        } else {
            Err(MessageError::Cbor)
        }
    }
}

pub trait MessageReceiver {
    async fn receive_message(&mut self) -> Result<Message, MessageError>;
}

impl<'d, D> MessageReceiver for Receiver<'d, D>
where
    D: Driver<'d>,
{
    async fn receive_message(&mut self) -> Result<Message, MessageError> {
        let mut packet_buf = [0u8; SERIAL_PACKET_SIZE as usize];
        let mut msg_buf = Vec::new();

        let mut counter = 0;
        let mut n = 0;

        while let Ok(len) = self.read_packet(&mut packet_buf).await {
            msg_buf.push(packet_buf);
            packet_buf.fill(0);

            if len < SERIAL_PACKET_SIZE.into() {
                // defmt::info!("{:?}", len);
                n = len;
                break;
            }

            counter += 1;
        }

        let msg_buf = msg_buf.into_iter().flatten().collect::<Vec<_>>();

        // defmt::info!("{:?}", msg_buf.len());

        defmt::info!("{:?}", msg_buf[..n]);
        if let Ok(msg) = Message::deserialize(&msg_buf[..n]) {
            Ok(msg)
        } else {
            Err(MessageError::Cbor)
        }
    }
}

// impl Message {
//     pub async fn tx_to_server<'a, D>(self, class: &mut CdcAcmClass<'a, D>) -> Result<(), MessageError>
//     where
//         D: Driver<'a>,
//     {
//         if let Ok(msg_cbor) = self.serialize() {
//             for chunk in msg_cbor.chunks(SERIAL_PACKET_SIZE.into()) {
//                 class.write_packet(chunk).await.map_err(|_| MessageError::USBWriteFailure)?;
//             }
//             class.write_packet(&[]).await.map_err(|_| MessageError::USBWriteFailure)?;

//             Ok(())
//         } else {
//             Err(MessageError::Cbor)
//         }
//     }

//     pub async fn rx_from_server<'a, D>(class: &mut CdcAcmClass<'a, D>) -> Result<Message, MessageError>
//     where
//         D: Driver<'a>,
//     {
//         let mut packet_buf = [0u8; SERIAL_PACKET_SIZE as usize];
//         let mut msg_buf = Vec::new();

//         let mut counter = 0;
//         let mut n = 0;

//         while let Ok(len) = class.read_packet(&mut packet_buf).await {
//             msg_buf.push(packet_buf);
//             packet_buf.fill(0);

//             if len < SERIAL_PACKET_SIZE.into() {
//                 n = len;
//                 // break out once we see a packet is not MAX SIZE (this means the message is complete)
//                 break;
//             }
//             counter += 1;
//         }

//         let msg_buf = msg_buf.into_iter().flatten().collect::<Vec<_>>();

//         if let Ok(msg) = Message::deserialize(&msg_buf[..]) {
//             Ok(msg)
//         } else {
//             // let msg = String::from_utf8_lossy(&msg_buf);
//             // if let Some(n) = msg.find('\0') {
//             //     cond_log!(warn!(
//             //         "Failed to deserialize message:\n\tData as String: {:?}",
//             //         msg.split_at(n).0
//             //     ));
//             // }
//             Err(MessageError::Cbor)
//         }
//     }
// }
