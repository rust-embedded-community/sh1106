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

use crate::{
    command::Page, displayrotation::DisplayRotation, interface::DisplayInterface,
    mode::displaymode::DisplayModeTrait, properties::DisplayProperties, Error,
};
use embedded_graphics_core::{prelude::Point, primitives::Rectangle};
use hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

/// What to clear.
#[derive(Debug, Copy, Clone)]
pub enum Clear {
    /// Clear the display buffer only, leaving the display contents alone.
    Buffer,

    /// Clear both the buffer and display.
    BufferAndDisplay,
}

// const BUFFER_SIZE: usize = 132 * 64 / 8;
const W: u32 = 132;
const H: u32 = 64;

/// Graphics mode handler
pub struct GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    properties: DisplayProperties<DI>,
    buffer: PackedBuffer<W, H, { (W * H / 8) as usize }>,
}

impl<DI> DisplayModeTrait<DI> for GraphicsMode<DI>
where
    DI: DisplayInterface,
{
    /// Create new GraphicsMode instance
    fn new(properties: DisplayProperties<DI>) -> Self {
        GraphicsMode {
            properties,
            buffer: PackedBuffer::new(),
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
    /// Clear the display buffer.
    pub fn clear(&mut self, clear: Clear) -> Result<(), DI::Error> {
        self.buffer = PackedBuffer::new();

        if matches!(clear, Clear::BufferAndDisplay) {
            let display_size = self.properties.get_size();
            let column_offset = display_size.column_offset();

            for i in 0..8 {
                self.properties
                    .draw_page(Page::from(i * 8), column_offset, &[0x00; 128])?;
            }
        }

        Ok(())
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

    /// Write out data to display.
    pub fn flush(&mut self) -> Result<(), DI::Error> {
        let display_size = self.properties.get_size();

        let active = self.buffer.active_area().intersection(&self.bounding_box());
        let start_page = (active.top_left.y / 8) as u8;
        let start_column = active.top_left.x as u8;

        let column_offset = display_size.column_offset();

        for (i, block) in self.buffer.active_blocks().enumerate() {
            let page = Page::from((start_page + i as u8) * 8);

            self.properties
                .draw_page(page, column_offset + start_column, block)?;
        }

        Ok(())
    }

    /// Turn a pixel on or off. A non-zero `value` is treated as on, `0` as off. If the X and Y
    /// coordinates are out of the bounds of the display, this method call is a noop.
    pub fn set_pixel(&mut self, x: u32, y: u32, value: u8) {
        let display_rotation = self.properties.get_rotation();

        let point = match display_rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => Point::new(x as i32, y as i32),
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                Point::new(y as i32, x as i32)
            }
        };

        self.buffer.set_pixel(
            point,
            if value == 0 {
                BinaryColor::Off
            } else {
                BinaryColor::On
            },
        )
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
use packed_display_buffer::PackedBuffer;

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

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        self.buffer.fill_solid(area, color)
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        self.buffer.fill_contiguous(area, colors)
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
