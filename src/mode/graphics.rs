//! Buffered display module for use with the [embedded-graphics] crate
//!
//! ```rust,no_run
//! use embedded_graphics::{
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     primitives::{Circle, Line, PrimitiveStyle, Rectangle},
//! };
//! use sh1106::{prelude::*, Builder};
//! # let i2c = sh1106::test_helpers::I2cStub;
//!
//! let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
//!
//! display.init().unwrap();
//! display.flush().unwrap();
//!
//! Line::new(Point::new(8, 16 + 16), Point::new(8 + 16, 16 + 16))
//!     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
//!     .draw(&mut display)
//!     .unwrap();
//!
//! Line::new(Point::new(8, 16 + 16), Point::new(8 + 8, 16))
//!     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
//!     .draw(&mut display)
//!     .unwrap();
//!
//! Line::new(Point::new(8 + 16, 16 + 16), Point::new(8 + 8, 16))
//!     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
//!     .draw(&mut display)
//!     .unwrap();
//!
//! Rectangle::with_corners(Point::new(48, 16), Point::new(48 + 16, 16 + 16))
//!     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
//!     .draw(&mut display)
//!     .unwrap();
//!
//! Circle::new(Point::new(88, 16), 16)
//!     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
//!     .draw(&mut display)
//!     .unwrap();
//!
//! display.flush().unwrap();
//! ```

use hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

use crate::{
    displayrotation::DisplayRotation, interface::DisplayInterface,
    mode::displaymode::DisplayModeTrait, properties::DisplayProperties, Error,
};

const BUFFER_SIZE: usize = 132 * 64 / 8;

/// Graphics mode handler
pub struct GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    properties: DisplayProperties<DI>,
    buffer: [u8; BUFFER_SIZE],
}

impl<DI> DisplayModeTrait<DI> for GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    /// Create new GraphicsMode instance
    fn new(properties: DisplayProperties<DI>) -> Self {
        GraphicsMode {
            properties,
            buffer: [0; BUFFER_SIZE],
        }
    }

    /// Release all resources used by GraphicsMode
    fn release(self) -> DisplayProperties<DI> {
        self.properties
    }
}

impl<DI> GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    /// Clear the display buffer. You need to call `display.flush()` for any effect on the screen
    pub fn clear(&mut self) {
        self.buffer = [0; BUFFER_SIZE];
    }

    /// Reset display
    pub fn reset<RST, DELAY, PinE>(
        &mut self,
        rst: &mut RST,
        delay: &mut DELAY,
    ) -> Result<(), Error<(), PinE>>
    where
        RST: OutputPin<Error = PinE>,
        DELAY: DelayMs<u8>,
    {
        rst.set_high().map_err(Error::Pin)?;
        delay.delay_ms(1);
        rst.set_low().map_err(Error::Pin)?;
        delay.delay_ms(10);
        rst.set_high().map_err(Error::Pin)
    }

    /// Write out data to display
    pub fn flush(&mut self) -> Result<(), DI::Error> {
        let display_size = self.properties.get_size();

        // Ensure the display buffer is at the origin of the display before we send the full frame
        // to prevent accidental offsets
        let (display_width, display_height) = display_size.dimensions();
        let column_offset = display_size.column_offset();
        self.properties.set_draw_area(
            (column_offset, 0),
            (display_width + column_offset, display_height),
        )?;

        let length = (display_width as usize) * (display_height as usize) / 8;

        self.properties.draw(&self.buffer[..length])
    }

    /// Turn a pixel on or off. A non-zero `value` is treated as on, `0` as off. If the X and Y
    /// coordinates are out of the bounds of the display, this method call is a noop.
    pub fn set_pixel(&mut self, x: u32, y: u32, value: u8) {
        let (display_width, _) = self.properties.get_size().dimensions();
        let display_rotation = self.properties.get_rotation();

        let idx = match display_rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                if x >= display_width as u32 {
                    return;
                }
                ((y as usize) / 8 * display_width as usize) + (x as usize)
            }

            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                if y >= display_width as u32 {
                    return;
                }
                ((x as usize) / 8 * display_width as usize) + (y as usize)
            }
        };

        if idx >= self.buffer.len() {
            return;
        }

        let (byte, bit) = match display_rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                let byte =
                    &mut self.buffer[((y as usize) / 8 * display_width as usize) + (x as usize)];
                let bit = 1 << (y % 8);

                (byte, bit)
            }
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                let byte =
                    &mut self.buffer[((x as usize) / 8 * display_width as usize) + (y as usize)];
                let bit = 1 << (x % 8);

                (byte, bit)
            }
        };

        if value == 0 {
            *byte &= !bit;
        } else {
            *byte |= bit;
        }
    }

    /// Display is set up in column mode, i.e. a byte walks down a column of 8 pixels from
    /// column 0 on the left, to column _n_ on the right
    pub fn init(&mut self) -> Result<(), DI::Error> {
        self.properties.init_column_mode()
    }

    /// Get display dimensions, taking into account the current rotation of the display
    pub fn get_dimensions(&self) -> (u8, u8) {
        self.properties.get_dimensions()
    }

    /// Set the display rotation
    pub fn set_rotation(&mut self, rot: DisplayRotation) -> Result<(), DI::Error> {
        self.properties.set_rotation(rot)
    }

    /// Set the display contrast
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DI::Error> {
        self.properties.set_contrast(contrast)
    }
}

#[cfg(feature = "graphics")]
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::Size,
    geometry::{Dimensions, OriginDimensions},
    pixelcolor::BinaryColor,
    Pixel,
};

#[cfg(feature = "graphics")]
impl<DI> DrawTarget for GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let bb = self.bounding_box();

        pixels
            .into_iter()
            .filter(|Pixel(pos, _color)| bb.contains(*pos))
            .for_each(|Pixel(pos, color)| {
                self.set_pixel(pos.x as u32, pos.y as u32, color.is_on().into())
            });

        Ok(())
    }
}

#[cfg(feature = "graphics")]
impl<DI> OriginDimensions for GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    fn size(&self) -> Size {
        let (w, h) = self.get_dimensions();

        Size::new(w.into(), h.into())
    }
}
