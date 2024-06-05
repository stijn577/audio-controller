#![no_std]
#![no_main]
#![deny(unsafe_code)]
#![allow(unstable_features)]
// #![allow(unused)]

extern crate alloc;
extern crate core;

// link defmt_rtt and use panic probe to print stack trace when panic! occurs
use defmt_rtt as _;
use panic_probe as _;

use defmt::info;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_stm32::{
    bind_interrupts,
    gpio::{Level, Output, Speed},
    peripherals, rcc, usart,
    usb_otg::{self, Driver, Instance},
    Config,
};
use embassy_time::Timer;
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
    Builder,
};

// defining our own modules
mod error;
mod prelude;
mod utils;

// use our own modules
use crate::utils::setup::*;

bind_interrupts!(
    // map the peripherals that need interrupt handlers
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

    let mut ep_out_buffer = [0u8; 256];
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

    let mut config = embassy_usb::Config::new(0xffff, 0x0000);

    config.manufacturer = Some("stijn577");
    config.product = Some("audio-controller");
    config.serial_number = Some("123456789");

    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];

    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut device_descriptor,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [],
        &mut control_buf,
    );

    let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);

    let mut usb = builder.build();

    let usb_fut = usb.run();

    // let button = ExtiInput::new(Input::new(pp.PC13, Pull::Down), pp.EXTI13);
    let led = Output::new(pp.PC13, Level::High, Speed::Low);

    info!("Pins set!");

    #[allow(unreachable_code)]
    let write_fut = async {
        class.wait_connection().await;
        info!("USB connected!");
        match echo(&mut class).await {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
        info!("Disconnected");
    };

    s.spawn(blinky_task(led)).ok();

    join(usb_fut, write_fut).await;

    unreachable!("REACHED END OF MAIN!");
}

#[embassy_executor::task]
async fn blinky_task(mut led: Output<'static, embassy_stm32::peripherals::PC13>) {
    loop {
        led.set_high();
        info!("LED off!");

        Timer::after_millis(1000).await;

        led.set_low();
        info!("LED on!");

        Timer::after_millis(1000).await;
    }
}

#[derive(thiserror_no_std::Error)]
struct Disconnected {
    #[source]
    _source: EndpointError,
}

impl From<EndpointError> for Disconnected {
    fn from(value: EndpointError) -> Self {
        match value {
            EndpointError::BufferOverflow => panic!("Buffer overflowed!"),
            EndpointError::Disabled => Disconnected { _source: value },
        }
    }
}

async fn echo<'d, T: Instance + 'd>(
    class: &mut CdcAcmClass<'d, Driver<'d, T>>,
) -> Result<(), Disconnected> {
    let mut buf = [0; 64];
    loop {
        let n = class.read_packet(&mut buf).await?;
        let data = &buf[..n];
        info!("data: {:x}", data);
        class.write_packet(data).await?;
    }
}
