use defmt::{info, println};
use embassy_stm32::usb_otg::Driver;
use embassy_stm32::usb_otg::Instance;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::Receiver;
use embassy_sync::channel::Sender;
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::driver::EndpointError;
use shared_data::message::Message;

use crate::CONFIG_ENTRIES;

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

pub(crate) async fn usb_send_message<'d, T: Instance + 'd>(
    class: &mut CdcAcmClass<'d, Driver<'d, T>>,
    rx: Receiver<'d, NoopRawMutex, Message, 5>,
) -> Result<(), Disconnected> {
    loop {
        let msg = rx.receive().await;
        info!("Sending messaging to server: {:?}", msg);
        let payload = msg.serialize().unwrap();

        for packet in payload.chunks(64) {
            class.write_packet(packet).await?
        }

        info!("Message sent succesfully")
    }
}

#[embassy_executor::task]
pub async fn mock_message_sender(tx: Sender<'static, NoopRawMutex, Message, 5>) {
    loop {
        println!("Sending mock message down the channel...");
        let msg = Message::ConfigEntry(CONFIG_ENTRIES[0].clone());
        tx.send(msg).await;
    }
}
