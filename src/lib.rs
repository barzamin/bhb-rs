#![no_std]

extern crate atsamd_hal as hal;

// rt feature: pull in cortex-m-rt and reexport entry macro
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
use hal::*;

pub use hal::target_device as pac;

define_pins!(
	struct Pins,
	target_device: target_device,

	/// The status LED pin
	pin led = a17,

	/// SPI MOSI for the external flash
	pin flash_mosi = pb22,
	/// SPI MISO for the external flash
	pin flash_miso = pb03,
	/// SPI SCK for the external flash
	pin flash_sck  = pb23,
	/// SPI chip select for the external flash
	pin flash_cs   = pa27,

    /// The USB D- pad
    pin usb_dm = a24,
    /// The USB D+ pad
    pin usb_dp = a25,
);

