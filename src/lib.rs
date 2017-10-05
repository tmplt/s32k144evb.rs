#![cfg_attr(any(feature = "panic-over-itm", feature = "panic-over-serial"), feature(core_intrinsics))]
#![cfg_attr(any(feature = "panic-over-itm", feature = "panic-over-serial"), feature(lang_items))]

#![no_std]


extern crate s32k144;
#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate bit_field;
extern crate embedded_types;

pub mod led;
pub mod wdog;
pub mod can;
pub mod lpuart;

#[macro_use]
#[cfg(feature = "serial")]
pub mod serial;

#[cfg(feature = "itm")]
mod itm;
