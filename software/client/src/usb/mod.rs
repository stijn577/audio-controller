use core::mem::MaybeUninit;

use defmt::info;
use defmt::warn;
use device_handler::MyDeviceHandler;
use embassy_futures::join::join;
use embassy_stm32::peripherals as pp;
use embassy_stm32::usb_otg;
use embassy_stm32::usb_otg::Driver;
use embassy_stm32::usb_otg::Instance;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::Receiver;
use embassy_sync::channel::Sender;
use embassy_usb::class::cdc_acm;
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::class::hid;
use embassy_usb::class::hid::HidWriter;
use embassy_usb::driver::EndpointError;
use embassy_usb::Builder;
use embassy_usb::UsbDevice;
use shared_data::action::Action;
use shared_data::message::usb_nostd::MessageReceiver;
use shared_data::message::usb_nostd::MessageSender;
use shared_data::message::Message;
use shared_data::HID_PACKET_SIZE;
use shared_data::SERIAL_PACKET_SIZE;
use usbd_hid::descriptor::MediaKey;
use usbd_hid::descriptor::MediaKeyboardReport;
use usbd_hid::descriptor::SerializedDescriptor;

use crate::Irqs;

pub(crate) mod device_handler;

pub struct Buffers {
    ep_out_buffer: [u8; 256],
    config_descriptor: [u8; 256],
    bos_descriptor: [u8; 256],
    control_buf: [u8; 64],
}

impl Default for Buffers {
    fn default() -> Self {
        Self {
            ep_out_buffer: [0u8; 256],
            config_descriptor: [0u8; 256],
            bos_descriptor: [0u8; 256],
            control_buf: [0u8; 64],
        }
    }
}

#[derive(Default)]
pub struct States<'d> {
    serial_state: cdc_acm::State<'d>,
    hid_state: hid::State<'d>,
}

pub struct USBCompositeDevice<'d> {
    serial: CdcAcmClass<'d, Driver<'d, pp::USB_OTG_FS>>,
    hid: HidWriter<'d, Driver<'d, pp::USB_OTG_FS>, { HID_PACKET_SIZE as usize }>,
}

impl<'d> USBCompositeDevice<'d> {
    pub(crate) fn new(
        pins: (pp::USB_OTG_FS, pp::PA12, pp::PA11),
        buffers: &'d mut Buffers,
        states: &'d mut States<'d>,
        driver_cfg: usb_otg::Config,
        builder_cfg: embassy_usb::Config<'d>,
        hid_cfg: hid::Config<'d>,
    ) -> (USBCompositeDevice<'d>, UsbDevice<'d, Driver<'d, pp::USB_OTG_FS>>) {
        // #[allow(unsafe_code)]
        // let mut usb_composite: USBCompositeDevice<'d> =
        //     unsafe { MaybeUninit::uninit().assume_init() };
        // usb_composite.device_handler = MyDeviceHandler::new();

        let driver = Driver::new_fs(pins.0, Irqs, pins.1, pins.2, &mut buffers.ep_out_buffer, driver_cfg);

        let mut builder = Builder::new(
            driver,
            builder_cfg,
            &mut buffers.config_descriptor,
            &mut buffers.bos_descriptor,
            &mut [],
            &mut buffers.control_buf,
        );

        // builder.handler(&mut usb_composite.device_handler);

        let serial = CdcAcmClass::new(&mut builder, &mut states.serial_state, SERIAL_PACKET_SIZE);
        let hid = HidWriter::new(&mut builder, &mut states.hid_state, hid_cfg);

        let usb_composite = Self { serial, hid };

        (usb_composite, builder.build())
    }

    pub(crate) async fn serial_run(mut self) -> ! {
        self.serial.wait_connection().await;

        let (mut tx, mut rx) = self.serial.split();

        loop {
            let msg = rx.receive_message().await;

            if let Ok(msg) = msg {
                info!("serial: {:?}", msg);
                tx.send_message(msg).await;
            } else {
                info!("No message received");
            }
        }
    }
}

pub(crate) async fn usb_serial<'d, D>(serial: &mut CdcAcmClass<'d, Driver<'d, D>>) -> Result<(), EndpointError>
where
    D: Instance,
{
    // info!("Waiting to receive...");
    // match Message::rx_from_server(serial).await {
    //     Ok(msg) => {
    //         info!("Message received! {:#?}", msg);
    //         info!("Echoing message...");
    //         if let Ok(_) = msg.tx_to_server(serial).await {
    //             info!("Message echoed!");
    //         } else {
    //             warn!("Echo failed")
    //         }
    //     }
    //     Err(e) => warn!("{:?}", e),
    // }

    Ok(())
}

pub(crate) async fn usb_hid<'d, D, const N: usize>(hid: &mut HidWriter<'d, Driver<'d, D>, N>) -> Result<(), EndpointError>
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
