#[cfg(all(feature = "std", not(feature = "defmt")))]
#[allow(unused_imports)]
pub(crate) use log::{debug, error, info, trace, warn};
#[cfg(all(feature = "std", not(feature = "defmt")))]
#[allow(unused_imports)]
pub(crate) use std::println;

#[cfg(all(feature = "defmt", not(feature = "std")))]
#[allow(unused_imports)]
pub(crate) use defmt::{debug, error, info, println, trace, warn};

#[cfg(feature = "embassy")]
pub(crate) use embassy_usb::{class::cdc_acm::CdcAcmClass, driver::Driver};
