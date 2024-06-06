#![no_std]
#![no_main]
#![deny(unsafe_code)]
#![allow(unstable_features)]

extern crate alloc;
extern crate core;

// defining our own modules
mod error;
mod message;
mod prelude;
mod utils;

use crate::utils::setup::*;
use alloc::boxed::Box;
use alloc::vec::Vec;
use defmt::info;
#[allow(unused)]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_stm32::{
    bind_interrupts,
    gpio::{Level, Output, Speed},
    peripherals, rcc, usart,
    usb_otg::{self, Driver},
    Config,
};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::Timer;
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    Builder,
};
use panic_probe as _;
use shared_data::message::config_entry::ConfigEntry;
use shared_data::message::Message;
use shared_data::N_CONFIG_ENTRIES;

const CONFIG_DEFAULT: ConfigEntry = ConfigEntry::Command(0, Vec::new());
static CONFIG_ENTRIES: [ConfigEntry; N_CONFIG_ENTRIES] = [CONFIG_DEFAULT; N_CONFIG_ENTRIES];

// map the peripherals that need interrupt handlers
bind_interrupts!(
    pub(crate) struct Irqs {
        USART1 => usart::InterruptHandler<peripherals::USART1>;
        USART6 => usart::InterruptHandler<peripherals::USART6>;
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

    let (mut class, mut usb) = {
        let ep_out_buffer = Box::leak(Box::new([0u8; 256]));
        let mut config = usb_otg::Config::default();
        config.vbus_detection = false;

        let driver = Driver::new_fs(pp.USB_OTG_FS, Irqs, pp.PA12, pp.PA11, ep_out_buffer, config);

        let mut config = embassy_usb::Config::new(0xffff, 0x0000);

        config.manufacturer = Some("stijn577");
        config.product = Some("audio-controller");
        config.serial_number = Some("123456789");

        config.device_class = 0xEF;
        config.device_sub_class = 0x02;
        config.device_protocol = 0x01;
        config.composite_with_iads = true;

        let device_descriptor = Box::leak(Box::new([0; 256]));
        let config_descriptor = Box::leak(Box::new([0; 256]));
        let bos_descriptor = Box::leak(Box::new([0; 256]));
        let control_buf = Box::leak(Box::new([0; 64]));

        let state = Box::leak(Box::new(State::new()));

        let mut builder = Builder::new(
            driver,
            config,
            device_descriptor,
            config_descriptor,
            bos_descriptor,
            &mut [],
            control_buf,
        );

        (CdcAcmClass::new(&mut builder, state, 64), builder.build())
    };

    let usb_fut = usb.run();
    // let button = ExtiInput::new(Input::new(pp.PC13, Pull::Down), pp.EXTI13);
    let led = Output::new(pp.PC13, Level::High, Speed::Low);

    info!("Pins set!");

    // we need to box leak to fix lifetimes
    let usb_message_channel = Box::leak(Box::new(Channel::<NoopRawMutex, Message, 5>::new()));

    #[allow(unreachable_code)]
    let write_fut = async {
        loop {
            class.wait_connection().await;
            info!("USB connected!");
            match message::usb_send_message(&mut class, usb_message_channel.receiver()).await {
                Ok(_) => todo!(),
                Err(_) => todo!(),
            }
            info!("Disconnected");
        }
    };

    s.spawn(blinky_task(led)).ok();
    s.spawn(message::mock_message_sender(usb_message_channel.sender()))
        .ok();

    join(usb_fut, write_fut).await;

    unreachable!("REACHED END OF MAIN!");
}

#[embassy_executor::task]
async fn blinky_task(mut led: Output<'static, peripherals::PC13>) {
    loop {
        led.set_high();
        info!("LED off!");

        Timer::after_millis(1000).await;

        led.set_low();
        info!("LED on!");

        Timer::after_millis(1000).await;
    }
}
