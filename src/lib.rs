//! sh1106 OLED display driver
//!
//! The driver must be initialised by passing an I2C or SPI interface peripheral to the
//! [`Builder`](builder/struct.Builder.html),
//! which will in turn create a driver instance in a particular mode. By default, the builder
//! returns a `mode::RawMode` instance which isn't very useful by itself. You can coerce the driver
//! into a more useful mode by calling `into()` and defining the type you want to coerce to. For
//! example, to initialise the display with an I2C interface and
//! [`mode::GraphicsMode`](mode/graphics/struct.GraphicsMode.html), you would do something like
//! this:
//!
//! ```rust,no_run
//! use sh1106::{prelude::*, Builder};
//! # let i2c = sh1106::test_helpers::I2cStub;
//!
//! let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
//!
//! display.init().unwrap();
//! display.flush().unwrap();
//!
//! display.set_pixel(10, 20, 1);
//!
//! display.flush().unwrap();
//! ```
//!
//! See the [example](https://github.com/jamwaffles/sh1106/blob/master/examples/graphics_i2c.rs)
//! for more usage. The [entire `embedded_graphics` featureset](https://github.com/jamwaffles/embedded-graphics#features)
//! is supported by this driver.
//!
//! It's possible to customise the driver to suit your display/application. Take a look at the
//! [Builder] for available options.
//!
//! # Examples
//!
//! Examples can be found in
//! [the examples/ folder](https://github.com/jamwaffles/sh1106/blob/master/examples)
//!
//! ## Draw some text to the display
//!
//! Uses [mode::GraphicsMode] and [embedded_graphics](../embedded_graphics/index.html).
//!
//! ```rust,no_run
//! use embedded_graphics::{
//!     mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     text::{Baseline, Text},
//! };
//! use sh1106::{prelude::*, Builder};
//! # let i2c = sh1106::test_helpers::I2cStub;
//!
//! let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
//!
//! display.init().unwrap();
//! display.flush().unwrap();
//!
//! let text_style = MonoTextStyleBuilder::new()
//!     .font(&FONT_6X10)
//!     .text_color(BinaryColor::On)
//!     .build();
//!
//! Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
//!     .draw(&mut display)
//!     .unwrap();
//!
//! Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
//!     .draw(&mut display)
//!     .unwrap();
//!
//! display.flush().unwrap();
//! ```

#![no_std]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

/// Errors in this crate
#[derive(Debug)]
pub enum Error<CommE, PinE> {
    /// Communication error
    Comm(CommE),
    /// Pin setting error
    Pin(PinE),
}

extern crate embedded_hal as hal;

pub mod builder;
mod command;
pub mod displayrotation;
pub mod displaysize;
pub mod interface;
pub mod mode;
pub mod prelude;
pub mod properties;
#[doc(hidden)]
pub mod test_helpers;

pub use crate::builder::{Builder, NoOutputPin};
