#![no_std]
#![no_main]
#![deny(unsafe_code)]
#![allow(unstable_features)]

extern crate alloc;
extern crate core;

// defining our own modules
mod error;
mod prelude;
mod usb;
mod utils;

use defmt_rtt as _;
use panic_probe as _;
use shared_data::{N_HWB, N_SWB, USB_PACKET_SIZE};

use crate::utils::setup::*;
use core::sync::atomic::{AtomicBool, Ordering};
use defmt::info;
use defmt::warn;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_stm32::{
    bind_interrupts,
    gpio::{Level, Output, Speed},
    peripherals, rcc,
    usb_otg::{self, Driver},
    Config,
};
use embassy_time::Timer;
use embassy_usb::{
    class::{
        cdc_acm::{self, CdcAcmClass},
        hid::{self, HidWriter},
    },
    Builder, Handler,
};
use shared_data::config::btn::BtnConfig;
use usb::usb_hid;
use usbd_hid::descriptor::{MediaKeyboardReport, SerializedDescriptor};

static HARDWARE_BUTTONS: BtnConfig<{ N_HWB }> = BtnConfig::new(heapless::Vec::new());
static SOFTWARE_BUTTONS: BtnConfig<{ N_SWB }> = BtnConfig::new(heapless::Vec::new());
// static SLIDERS: SliderConfig = SliderConfig::new(hl::Vec::new());

// const USB_PACKET_SIZE: usize = 64;
// static USB_RX_CHANNEL: Channel<CriticalSectionRawMutex, Message, 5> = Channel::new();
// static USB_TX_CHANNEL: Channel<CriticalSectionRawMutex, Message, 5> = Channel::new();

// map the peripherals that need interrupt handlers
bind_interrupts!(
    pub(crate) struct Irqs {
        OTG_FS => usb_otg::InterruptHandler<peripherals::USB_OTG_FS>;
    }
);

#[allow(unreachable_code)]
#[embassy_executor::main]
async fn main(s: Spawner) {
    heap_setup();

    let mut clock_config = rcc::Config::default();
    clock_setup(&mut clock_config);

    let mut embassy_config = Config::default();
    embassy_config.rcc = clock_config;

    let pp = embassy_stm32::init(embassy_config);

    info!("Basics done!");

    let mut ep_out_buffer = [0u8; 256];

    // let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    // let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 64];

    let mut serial_state = cdc_acm::State::new();
    let mut hid_state = hid::State::new();

    let mut device_handler = MyDeviceHandler::new();

    let (mut usb, mut serial, mut hid) = {
        let mut config = usb_otg::Config::default();
        config.vbus_detection = false;

        let driver = Driver::new_fs(
            pp.USB_OTG_FS,
            Irqs,
            pp.PA12,
            pp.PA11,
            &mut ep_out_buffer,
            config,
        );

        let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);

        // identifiers
        config.manufacturer = Some("stijn577");
        config.product = Some("audio-controller");
        config.serial_number = Some("stn577");

        // composite device settings
        config.device_class = 0xEF;
        config.device_sub_class = 0x02;
        config.device_protocol = 0x01;
        config.composite_with_iads = true;

        config.max_packet_size_0 = (USB_PACKET_SIZE as usize).try_into().unwrap();

        let mut builder = Builder::new(
            driver,
            config,
            &mut config_descriptor,
            &mut bos_descriptor,
            &mut [],
            &mut control_buf,
        );

        // builder.msos_descriptor(msos::windows_version::WIN10, 2);

        // register serial port member for composite class
        let serial = CdcAcmClass::new(&mut builder, &mut serial_state, 64);

        builder.handler(&mut device_handler);

        // config for hid keyboard
        let hid_config = hid::Config {
            report_descriptor: MediaKeyboardReport::desc(),
            request_handler: None,
            poll_ms: 60,
            max_packet_size: 8,
        };

        // register key hid member for composite class
        let hid: HidWriter<Driver<peripherals::USB_OTG_FS>, 8> =
            HidWriter::new(&mut builder, &mut hid_state, hid_config);

        let usb = builder.build();

        (usb, serial, hid)
    };

    let led = Output::new(pp.PC13, Level::High, Speed::Low);

    info!("Pins set!");

    let usb_run = usb.run();

    let key_hid_fut = async {
        loop {
            Timer::after_millis(1000).await;
            // if let Ok(_) = usb_hid(&mut hid).await {
            //     info!("Success")
            // } else {
            //     warn!("Failed to send message to server")
            // }
        }
    };

    #[allow(unreachable_code)]
    let serial_fut = async {
        serial.wait_connection().await;

        info!("USB connected!");

        loop {
            match usb::usb_serial(
                &mut serial,
                // USB_TX_CHANNEL.receiver(),
                // USB_RX_CHANNEL.sender(),
            )
            .await
            {
                Ok(_) => (),
                Err(_) => warn!("Failed to send message to server"),
            }
            Timer::after_millis(1000).await;
        }
    };

    s.spawn(blinky_task(led)).ok();

    join(usb_run, join(serial_fut, key_hid_fut)).await;

    unreachable!("REACHED END OF MAIN!");
}

#[embassy_executor::task]
async fn blinky_task(mut led: Output<'static, peripherals::PC13>) {
    loop {
        led.set_high();
        Timer::after_millis(1000).await;

        led.set_low();
        Timer::after_millis(1000).await;
    }
}

// struct MyRequestHandler {}

// impl RequestHandler for MyRequestHandler {
//     fn get_report(&mut self, id: ReportId, _buf: &mut [u8]) -> Option<usize> {
//         info!("Get report for {:?}", id);
//         None
//     }

//     fn set_report(&mut self, id: ReportId, data: &[u8]) -> OutResponse {
//         info!("Set report for {:?}: {=[u8]}", id, data);
//         OutResponse::Accepted
//     }

//     fn set_idle_ms(&mut self, id: Option<ReportId>, dur: u32) {
//         info!("Set idle rate for {:?} to {:?}", id, dur);
//     }

//     fn get_idle_ms(&mut self, id: Option<ReportId>) -> Option<u32> {
//         info!("Get idle rate for {:?}", id);
//         None
//     }
// }

struct MyDeviceHandler {
    configured: AtomicBool,
}

impl MyDeviceHandler {
    fn new() -> Self {
        MyDeviceHandler {
            configured: AtomicBool::new(false),
        }
    }
}

impl Handler for MyDeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        self.configured.store(false, Ordering::Relaxed);
        if enabled {
            info!("Device enabled");
        } else {
            info!("Device disabled");
        }
    }

    fn reset(&mut self) {
        self.configured.store(false, Ordering::Relaxed);
        info!("Bus reset, the Vbus current limit is 100mA");
    }

    fn addressed(&mut self, addr: u8) {
        self.configured.store(false, Ordering::Relaxed);
        info!("USB address set to: {}", addr);
    }

    fn configured(&mut self, configured: bool) {
        self.configured.store(configured, Ordering::Relaxed);
        if configured {
            info!(
                "Device configured, it may now draw up to the configured current limit from Vbus."
            )
        } else {
            info!("Device is no longer configured, the Vbus current limit is 100mA.");
        }
    }
}
