//! Interface factory
//!
//! This is the easiest way to create a driver instance. You can set various parameters of the
//! driver and give it an interface to use. The builder will return a
//! [`mode::RawMode`](../mode/raw/struct.RawMode.html) object which you should coerce to a richer
//! display mode, like [mode::Graphics](../mode/graphics/struct.GraphicsMode.html) for drawing
//! primitives and text.
//!
//! # Examples
//!
//! Connect over SPI with default rotation (0 deg) and size (128x64):
//!
//! ```rust,ignore
//! let spi = /* SPI interface from your HAL of choice */;
//! let dc = /* GPIO data/command select pin */;
//!
//! Builder::new().connect_spi(spi, dc);
//! ```
//!
//! Connect over I2C, changing lots of options
//!
//! ```rust,ignore
//! let i2c = /* I2C interface from your HAL of choice */;
//!
//! Builder::new()
//!     .with_rotation(DisplayRotation::Rotate180)
//!     .with_i2c_addr(0x3D)
//!     .with_size(DisplaySize::Display128x32)
//!     .connect_i2c(i2c);
//! ```
//!
//! The above examples will produce a [RawMode](../mode/raw/struct.RawMode.html) instance
//! by default. You need to coerce them into a mode by specifying a type on assignment. For
//! example, to use [`GraphicsMode` mode](../mode/graphics/struct.GraphicsMode.html):
//!
//! ```rust,ignore
//! let spi = /* SPI interface from your HAL of choice */;
//! let dc = /* GPIO data/command select pin */;
//!
//! let display: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();
//! ```

use crate::displayrotation::DisplayRotation;
use crate::displaysize::DisplaySize;
use crate::interface::{I2cInterface, SpiInterface};
use crate::mode::displaymode::DisplayMode;
use crate::mode::raw::RawMode;
use crate::properties::DisplayProperties;
use hal;
use hal::digital::v2::OutputPin;

/// Builder struct. Driver options and interface are set using its methods.
#[derive(Clone, Copy)]
pub struct Builder<CS = NoOutputPin> {
    display_size: DisplaySize,
    rotation: DisplayRotation,
    i2c_addr: u8,
    spi_cs: CS,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    /// Create new builder with a default size of 128 x 64 pixels and no rotation.
    pub fn new() -> Builder<NoOutputPin> {
        Builder {
            display_size: DisplaySize::Display128x64,
            rotation: DisplayRotation::Rotate0,
            i2c_addr: 0x3c,
            spi_cs: NoOutputPin,
        }
    }
}

impl<CS> Builder<CS>
where
    CS: OutputPin,
{
    /// Set the size of the display. Supported sizes are defined by [DisplaySize].
    pub fn with_size(self, display_size: DisplaySize) -> Self {
        Self {
            display_size,
            ..self
        }
    }

    /// Set the I2C address to use. Defaults to 0x3C which is the most common address.
    /// The other address specified in the datasheet is 0x3D. Ignored when using SPI interface.
    pub fn with_i2c_addr(self, i2c_addr: u8) -> Self {
        Self { i2c_addr, ..self }
    }

    /// Set the rotation of the display to one of four values. Defaults to no rotation.
    pub fn with_rotation(self, rotation: DisplayRotation) -> Self {
        Self { rotation, ..self }
    }

    /// Set the SPI chip select (CS) pin to use. The CS pin is not required for the controller for
    /// function, but can be used if the bus is shared with other devices. If not used, the CS pin
    /// on the controller should be connected to ground. Ignored when using I2C interface.
    pub fn with_spi_cs<NEWCS>(self, spi_cs: NEWCS) -> Builder<NEWCS>
    where
        NEWCS: OutputPin,
    {
        Builder {
            display_size: self.display_size,
            i2c_addr: self.i2c_addr,
            rotation: self.rotation,
            spi_cs,
        }
    }

    /// Finish the builder and use I2C to communicate with the display
    pub fn connect_i2c<I2C>(self, i2c: I2C) -> DisplayMode<RawMode<I2cInterface<I2C>>>
    where
        I2C: hal::blocking::i2c::Write,
    {
        let properties = DisplayProperties::new(
            I2cInterface::new(i2c, self.i2c_addr),
            self.display_size,
            self.rotation,
        );
        DisplayMode::<RawMode<I2cInterface<I2C>>>::new(properties)
    }

    /// Finish the builder and use SPI to communicate with the display
    pub fn connect_spi<SPI, DC>(
        self,
        spi: SPI,
        dc: DC,
    ) -> DisplayMode<RawMode<SpiInterface<SPI, DC, CS>>>
    where
        SPI: hal::blocking::spi::Transfer<u8> + hal::blocking::spi::Write<u8>,
        DC: OutputPin,
    {
        let properties = DisplayProperties::new(
            SpiInterface::new(spi, dc, self.spi_cs),
            self.display_size,
            self.rotation,
        );
        DisplayMode::<RawMode<SpiInterface<SPI, DC, CS>>>::new(properties)
    }
}

/// Represents an unused output pin.
#[derive(Clone, Copy)]
pub struct NoOutputPin;

impl OutputPin for NoOutputPin {
    type Error = ();

    fn set_low(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), ()> {
        Ok(())
    }
}
