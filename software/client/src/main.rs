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

use defmt_rtt as _;
use panic_probe as _;

use crate::utils::setup::*;
use defmt::info;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_stm32::{
    bind_interrupts,
    gpio::{Level, Output, Speed},
    peripherals, rcc, usart,
    usb_otg::{self, Driver},
    Config,
};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::Timer;
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    msos::{self, windows_version},
    types::InterfaceNumber,
    Builder,
};
use heapless as hl;
use shared_data::config::{hardware::HWBtnConfig, slider::SliderConfig, software::SWBtnConfig};
use shared_data::message::Message;

//TODO: static bitmap manager to interact with SD card
// ...
// static HARDWARE_BUTTONS: HWBtnConfig = HWBtnConfig::new(hl::Vec::new());
// static SOFTWARE_BUTTONS: SWBtnConfig = SWBtnConfig::new(hl::Vec::new());
// static SLIDERS: SliderConfig = SliderConfig::new(hl::Vec::new());

const USB_PACKET_SIZE: usize = 64;
static USB_RX_CHANNEL: Channel<CriticalSectionRawMutex, Message, 5> = Channel::new();
static USB_TX_CHANNEL: Channel<CriticalSectionRawMutex, Message, 5> = Channel::new();

// map the peripherals that need interrupt handlers
bind_interrupts!(
    pub(crate) struct Irqs {
        USART1 => usart::InterruptHandler<peripherals::USART1>;
        USART6 => usart::InterruptHandler<peripherals::USART6>;
        OTG_FS => usb_otg::InterruptHandler<peripherals::USB_OTG_FS>;
    }
);

const DEVICE_INTERFACE_GUIDS: &[&str] = &["{EAA9A5DC-30BA-44BC-9232-606CDC875321}"];

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

    let mut ep_out_buffer = [0u8; 256 * 5];

    let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 256];

    let mut state = State::new();

    // let allocator = usb_device::bus::UsbBusAllocator::new()
    // let mut serial = usbd_serial::SerialPort::new(&)

    let (mut usb, mut class) = {
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

        config.manufacturer = Some("stijn577");
        config.product = Some("audio-controller");
        config.serial_number = Some("12345678");

        config.device_class = 0xEF;
        config.device_sub_class = 0x02;
        config.device_protocol = 0x01;
        config.composite_with_iads = true;

        let mut builder = Builder::new(
            driver,
            config,
            // &mut device_descriptor,
            &mut config_descriptor,
            &mut bos_descriptor,
            &mut msos_descriptor,
            &mut control_buf,
        );

        builder.msos_descriptor(msos::windows_version::WIN10, 2);

        let class = CdcAcmClass::new(&mut builder, &mut state, 64);

        let msos_writer = builder.msos_writer();
        msos_writer.configuration(0);
        msos_writer.function(InterfaceNumber(0));
        msos_writer.function_feature(msos::CompatibleIdFeatureDescriptor::new("WINUSB", ""));
        msos_writer.function_feature(msos::RegistryPropertyFeatureDescriptor::new(
            "DeviceInterfaceGUIDs",
            msos::PropertyData::RegMultiSz(DEVICE_INTERFACE_GUIDS),
        ));

        let usb = builder.build();

        (usb, class)
    };

    let usb_run = usb.run();
    // let button = ExtiInput::new(Input::new(pp.PC13, Pull::Down), pp.EXTI13);
    let led = Output::new(pp.PC13, Level::High, Speed::Low);

    info!("Pins set!");

    #[allow(unreachable_code)]
    let usb_rx_tx = async {
        loop {
            class.wait_connection().await;
            info!("USB connected!");

            match message::usb_messaging(
                &mut class,
                USB_TX_CHANNEL.receiver(),
                USB_RX_CHANNEL.sender(),
            )
            .await
            {
                Ok(_) => todo!(),
                Err(_) => todo!(),
            }
            info!("Disconnected");
        }
    };

    s.spawn(blinky_task(led)).ok();

    join(usb_run, usb_rx_tx).await;

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
