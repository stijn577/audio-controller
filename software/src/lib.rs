#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod audiolevels;
pub mod config;
pub mod action;
pub mod message;

#[cfg(target_os = "windows")]
const _SHELL: &str = "powershell";
#[cfg(target_os = "windows")]
const _SHELL_EXEC: &str = "-Command";

#[cfg(target_os = "linux")]
const _SHELL: &str = "/bin/sh";
#[cfg(target_os = "linux")]
const _SHELL_EXEC: &str = "-c";

/// The number of hardware buttons.
pub const N_HWB: usize = 8;
/// The number of different screens
pub const N_SCREENS: usize = 3;
/// The number of software buttons per screen.
pub const N_SWB_PER_SCREEN: usize = 6;
/// The amount of sliders on the controller.
pub const N_SLIDERS: usize = 5;
/// The amount of bitmaps that can be shown on sliders
pub const N_SLIDERS_BMPS: usize = N_SLIDERS;
/// The amount of bitmaps on the controller.
pub const N_BITMAPS: usize = N_SWB_PER_SCREEN * N_SCREENS + N_SLIDERS_BMPS;
