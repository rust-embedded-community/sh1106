//! sh1106 Communication Interface (I2C/SPI)
//!
//! These are the two supported interfaces for communicating with the display. They're used by the
//! [builder](../builder/index.html) methods
//! [connect_i2c](../builder/struct.Builder.html#method.connect_i2c) and
//! [connect_spi](../builder/struct.Builder.html#method.connect_spi).
//!
//! The types that these interfaces define are quite lengthy, so it is recommended that you create
//! a type alias. Here's an example for the I2C1 on an STM32F103xx:
//!
//! ```rust
//! # use stm32f1xx_hal::gpio::gpiob::{PB8, PB9};
//! # use stm32f1xx_hal::gpio::{Alternate, OpenDrain};
//! # use stm32f1xx_hal::i2c::I2c;
//! # use stm32f1xx_hal::prelude::*;
//! # use stm32f1xx_hal::pac::I2C1;
//! # use sh1106::{interface::I2cInterface, mode::GraphicsMode, Builder};
//! type OledDisplay = GraphicsMode<
//!     I2cInterface<I2c<I2C1, (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>)>>,
//! >;
//! ```
//!
//! Here's one for SPI1 on an STM32F103xx:
//!
//! ```rust
//! # use stm32f1xx_hal::gpio::gpioa::{PA5, PA6, PA7};
//! # use stm32f1xx_hal::gpio::gpiob::PB1;
//! # use stm32f1xx_hal::gpio::{Alternate, Floating, Input, Output, PushPull};
//! # use stm32f1xx_hal::spi;
//! # use stm32f1xx_hal::spi::Spi;
//! # use stm32f1xx_hal::pac::SPI1;
//! # use sh1106::{interface::SpiInterface, mode::GraphicsMode};
//! pub type OledDisplay = GraphicsMode<
//!     SpiInterface<
//!         Spi<
//!             SPI1,
//!             spi::Spi1NoRemap,
//!             (
//!                 PA5<Alternate<PushPull>>,
//!                 PA6<Input<Floating>>,
//!                 PA7<Alternate<PushPull>>,
//!             ),
//!             u8
//!         >,
//!         PB1<Output<PushPull>>,
//!         sh1106::builder::NoOutputPin,
//!     >,
//! >;
//! ```

pub mod i2c;
pub mod spi;

/// A method of communicating with sh1106
pub trait DisplayInterface {
    /// Interface error type
    type Error;
    /// Underlying interface type
    type Interface;

    /// Initialize device.
    fn init(&mut self) -> Result<(), Self::Error>;
    /// Send a batch of up to 8 commands to display.
    fn send_commands(&mut self, cmd: &[u8]) -> Result<(), Self::Error>;
    /// Send data to display.
    fn send_data(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
    /// Release the interface
    fn release(self) -> Self::Interface;
}

pub use self::{i2c::I2cInterface, spi::SpiInterface};
