#![no_std]
#![no_main]
#![deny(unsafe_code)]
#![allow(unstable_features)]
// #![allow(unused)]

extern crate alloc;
extern crate core;

use alloc::{boxed::Box, vec};
// link defmt_rtt and use panic probe to print stack trace when panic! occurs
use defmt_rtt as _;
use embassy_time::Timer;
use panic_probe as _;

// use external crates
use cortex_m_rt::entry;
use defmt::info;
use embassy_executor::{Executor, Spawner};
use embassy_stm32::{
    bind_interrupts,
    exti::ExtiInput,
    gpio::{Input, Level, Output, Pull, Speed},
    interrupt::{self, InterruptExt, Priority},
    peripherals, rcc, usart,
    usb_otg::{self, Driver},
    Config,
};
use embassy_usb::{
    self,
    class::hid::{self, HidReaderWriter, State},
    Builder,
};
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

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

    let mut a = true;

    let mut embassy_config = Config::default();
    embassy_config.rcc = clock_config;

    let pp = embassy_stm32::init(embassy_config);

    info!("Basics done!");

    // let button = ExtiInput::new(Input::new(pp.PC13, Pull::Down), pp.EXTI13);
    let led = Output::new(pp.PC13, Level::High, Speed::Low);

    info!("Pins set!");

    s.spawn(blinky_task(led)).ok();

    unreachable!("REACHED END OF MAIN!");
}

#[embassy_executor::task]
async fn blinky_task(mut led: Output<'static, embassy_stm32::peripherals::PC13>) {
    let mut a = 0;
    loop {
        a += 1;

        led.set_high();
        info!("LED off!");

        Timer::after_millis(1000).await;

        led.set_low();
        info!("LED on!");

        Timer::after_millis(1000).await;
    }
}

// #[cfg(test)]
// #[cfg(not(bench))]
// #[defmt_test::tests]
// mod tests {
//     use defmt::{info, println};
//
//     #[test]
//     fn assert_ok() {
//         assert!(true);
//     }
//
//     #[test]
//     fn test_factorial() {
//         // assert_eq!(factorial(5), 120);
//     }
// }
