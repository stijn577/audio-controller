#![no_std]
#[cfg(feature = "std")]
extern crate std;

// define launcher for shell execution (otherwise it might hang, so use shell closest to OS)
extern crate alloc;

pub mod message;

#[cfg(target_os = "windows")]
const _SHELL: &str = "powershell";
#[cfg(target_os = "windows")]
const _SHELL_EXEC: &str = "-Command";

#[cfg(target_os = "linux")]
const _SHELL: &str = "/bin/sh";
#[cfg(target_os = "linux")]
const _SHELL_EXEC: &str = "-c";

pub const N_CONFIG_ENTRIES: usize = 10;
pub const N_SLIDERS: usize = 5;
