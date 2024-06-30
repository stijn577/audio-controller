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

use alloc::fmt::format;
use defmt_rtt as _;
use embassy_sync::blocking_mutex::CriticalSectionMutex;
use embassy_sync::mutex::Mutex;
use panic_probe as _;
use shared_data::config::slider::SliderConfig;
use shared_data::{N_HWB, N_SLIDERS, N_SWB, SERIAL_PACKET_SIZE};
use usb::device_handler::MyDeviceHandler;

use crate::utils::setup::*;
use core::cell::RefCell;
use core::fmt::Write;
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
use usb::{usb_hid, Buffers, States, USBCompositeDevice};
use usbd_hid::descriptor::{MediaKeyboardReport, SerializedDescriptor};

static mut HW_BTN_CFG: CriticalSectionMutex<RefCell<BtnConfig<N_HWB>>> = CriticalSectionMutex::new(RefCell::new(BtnConfig::new()));
static mut SW_BTN_CFG: CriticalSectionMutex<RefCell<BtnConfig<N_SWB>>> = CriticalSectionMutex::new(RefCell::new(BtnConfig::new()));
static mut SLIDER_CFG: CriticalSectionMutex<RefCell<SliderConfig<N_SLIDERS>>> = CriticalSectionMutex::new(RefCell::new(SliderConfig::new()));

// const USB_PACKET_SIZE: usize = 64;
// static USB_RX_CHANNEL: Channel<CriticalSectionRawMutex, Message, 5> = Channel::new();
// static USB_TX_CHANNEL: Channel<CriticalSectionRawMutex, Message, 5> = Channel::new();

// static mut CONFIG: shared_data::config::Config = shared_data::config::Config::new();

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

    let mut buffers = Buffers::default();
    let mut states = States::default();

    let mut driver_cfg = usb_otg::Config::default();
    driver_cfg.vbus_detection = false;

    let mut builder_cfg = embassy_usb::Config::new(0x0577, 0x0577);
    // identifiers
    builder_cfg.manufacturer = Some("stijn577");
    builder_cfg.product = Some("audio-controller");
    builder_cfg.serial_number = Some("stijn577");
    // composite device settings
    builder_cfg.device_class = 0xEF;
    builder_cfg.device_sub_class = 0x02;
    builder_cfg.device_protocol = 0x01;
    builder_cfg.composite_with_iads = true;

    let hid_cfg = hid::Config {
        report_descriptor: MediaKeyboardReport::desc(),
        request_handler: None,
        poll_ms: 60,
        max_packet_size: 8,
    };

    let (usb_man,mut  usb_dev) = USBCompositeDevice::new(
        (pp.USB_OTG_FS, pp.PA12, pp.PA11),
        &mut buffers,
        &mut states,
        driver_cfg,
        builder_cfg,
        hid_cfg,
    );

    let led = Output::new(pp.PC13, Level::High, Speed::Low);

    info!("Pins set!");

    let serial_fut = usb_man.serial_run();
    let usb_fut = usb_dev.run();

    // let usb_run = usb.run();

    // let key_hid_fut = async {
    //     loop {
    //         Timer::after_millis(1000).await;
    //         // if let Ok(_) = usb_hid(&mut hid).await {
    //         //     info!("Success")
    //         // } else {
    //         //     warn!("Failed to send message to server")
    //         // }
    //     }
    // };

    // #[allow(unreachable_code)]
    // let serial_fut = async {
    //     serial.wait_connection().await;

    //     info!("USB connected!");

    //     loop {
    //         match usb::usb_serial(
    //             &mut serial,
    //             // USB_TX_CHANNEL.receiver(),
    //             // USB_RX_CHANNEL.sender(),
    //         )
    //         .await
    //         {
    //             Ok(_) => (),
    //             Err(_) => warn!("Failed to send message to server"),
    //         }
    //         Timer::after_millis(1000).await;
    //     }
    // };

    s.spawn(blinky_task(led)).ok();

    // serial_fut.await;
    join(serial_fut, usb_fut).await;

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
