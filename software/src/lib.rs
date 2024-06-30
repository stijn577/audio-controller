#![no_std]
#![feature(debug_closure_helpers)]
#![feature(const_trait_impl)]
#![feature(effects)]

#[cfg(feature = "std")]
extern crate std;

// #[cfg(all(feature = "std", feature = "defmt"))]
// compile_error!("You can't use std and defmt features at the same time");

extern crate alloc;

#[macro_use]
pub(crate) mod macros;
pub mod action;
pub mod audiolevels;
pub mod config;
pub mod message;
mod prelude;

#[cfg(target_os = "windows")]
const _SHELL: &str = "powershell";
#[cfg(target_os = "windows")]
const _SHELL_EXEC: &str = "-Command";

#[cfg(target_os = "linux")]
const _SHELL: &str = "/bin/sh";
#[cfg(target_os = "linux")]
const _SHELL_EXEC: &str = "-c";

/// packet size for a serial usb packet
pub const SERIAL_PACKET_SIZE: u16 = 64;
/// packet size for a hid usb packet
pub const HID_PACKET_SIZE: u16 = 8;
/// The number of hardware buttons.
pub const N_HWB: usize = 8;
/// The amout of software button entries.
pub const N_SWB: usize = u8::MAX as usize;
/// The amount of sliders on the controller.
pub const N_SLIDERS: usize = 5;
