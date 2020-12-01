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

	/// The v/oct ADC pin
	pin a1 = b8,

	/// The audio out pin
	pin a0 = a2,

	/// The gate out pin
	pin d0 = a11,

	/// The gate in pin
	pin d2 = a14,

	/// SPI MOSI for the external flash
	pin flash_mosi = b22,
	/// SPI MISO for the external flash
	pin flash_miso = b3,
	/// SPI SCK for the external flash
	pin flash_sck  = b23,
	/// SPI chip select for the external flash
	pin flash_cs   = a27,

    /// The USB D- pad
    pin usb_dm = a24,
    /// The USB D+ pad
    pin usb_dp = a25,
);

