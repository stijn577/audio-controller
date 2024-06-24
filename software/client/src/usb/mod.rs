use defmt::info;
use defmt::warn;
use embassy_stm32::usb_otg::Driver;
use embassy_stm32::usb_otg::Instance;
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::class::hid::HidWriter;
use embassy_usb::driver::EndpointError;
use shared_data::message::Message;
use usbd_hid::descriptor::MediaKey;
use usbd_hid::descriptor::MediaKeyboardReport;

pub(crate) mod device_handler;

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
pub(crate) async fn usb_serial<'d, D>(
    serial: &mut CdcAcmClass<'d, Driver<'d, D>>,
) -> Result<(), EndpointError>
where
    D: Instance,
{
    info!("Waiting to receive...");
    match Message::rx_from_server(serial).await {
        Ok(msg) => {
            info!("Message received! {:#?}", msg);
            info!("Echoing message...");
            if let Ok(_) = msg.tx_to_server(serial).await {
                info!("Message echoed!");
            } else {
                warn!("Echo failed")
            }
        }
        Err(e) => warn!("{:?}", e),
    }

    Ok(())
}

pub(crate) async fn usb_hid<'d, D, const N: usize>(
    hid: &mut HidWriter<'d, Driver<'d, D>, N>,
) -> Result<(), EndpointError>
where
    D: Instance,
{
    info!("Writing keycodes!");

    let report = MediaKeyboardReport {
        usage_id: MediaKey::PlayPause.into(),
    };

    hid.write_serialize(&report).await
}

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
