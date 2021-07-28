//! Container to store and set display properties

use crate::{
    command::{Command, VcomhLevel},
    displayrotation::DisplayRotation,
    displaysize::DisplaySize,
    interface::DisplayInterface,
};

/// Display properties struct
pub struct DisplayProperties<DI> {
    iface: DI,
    display_size: DisplaySize,
    display_rotation: DisplayRotation,
    draw_area_start: (u8, u8),
    draw_area_end: (u8, u8),
    draw_column: u8,
    draw_row: u8,
}

impl<DI> DisplayProperties<DI>
where
    DI: DisplayInterface,
{
    /// Create new DisplayProperties instance
    pub fn new(
        iface: DI,
        display_size: DisplaySize,
        display_rotation: DisplayRotation,
    ) -> DisplayProperties<DI> {
        DisplayProperties {
            iface,
            display_size,
            display_rotation,
            draw_area_start: (0, 0),
            draw_area_end: (0, 0),
            draw_column: 0,
            draw_row: 0,
        }
    }

    /// Initialise the display in column mode (i.e. a byte walks down a column of 8 pixels) with
    /// column 0 on the left and column _(display_width - 1)_ on the right.
    pub fn init_column_mode(&mut self) -> Result<(), DI::Error> {
        self.iface.init()?;
        // TODO: Break up into nice bits so display modes can pick whathever they need
        let (_, display_height) = self.display_size.dimensions();
        let display_rotation = self.display_rotation;

        Command::DisplayOn(false).send(&mut self.iface)?;
        Command::DisplayClockDiv(0x8, 0x0).send(&mut self.iface)?;
        Command::Multiplex(display_height - 1).send(&mut self.iface)?;
        Command::DisplayOffset(0).send(&mut self.iface)?;
        Command::StartLine(0).send(&mut self.iface)?;
        // TODO: Ability to turn charge pump on/off
        // Display must be off when performing this command
        Command::ChargePump(true).send(&mut self.iface)?;

        self.set_rotation(display_rotation)?;

        match self.display_size {
            DisplaySize::Display128x32 => Command::ComPinConfig(false).send(&mut self.iface),
            DisplaySize::Display128x128
            | DisplaySize::Display128x64
            | DisplaySize::Display128x64NoOffset
            | DisplaySize::Display132x64 => Command::ComPinConfig(true).send(&mut self.iface),
        }?;

        Command::Contrast(0x80).send(&mut self.iface)?;
        Command::PreChargePeriod(0x1, 0xF).send(&mut self.iface)?;
        Command::VcomhDeselect(VcomhLevel::Auto).send(&mut self.iface)?;
        Command::AllOn(false).send(&mut self.iface)?;
        Command::Invert(false).send(&mut self.iface)?;
        Command::DisplayOn(true).send(&mut self.iface)?;

        Ok(())
    }

    /// Set the position in the framebuffer of the display where any sent data should be
    /// drawn. This method can be used for changing the affected area on the screen as well
    /// as (re-)setting the start point of the next `draw` call.
    pub fn set_draw_area(&mut self, start: (u8, u8), end: (u8, u8)) -> Result<(), DI::Error> {
        self.draw_area_start = start;
        self.draw_area_end = end;
        self.draw_column = start.0;
        self.draw_row = start.1;

        self.send_draw_address()
    }

    /// Send the data to the display for drawing at the current position in the framebuffer
    /// and advance the position accordingly. Cf. `set_draw_area` to modify the affected area by
    /// this method.
    pub fn draw(&mut self, mut buffer: &[u8]) -> Result<(), DI::Error> {
        while !buffer.is_empty() {
            let count = self.draw_area_end.0 - self.draw_column;
            self.iface.send_data(&buffer[..count as usize])?;
            self.draw_column += count;

            if self.draw_column >= self.draw_area_end.0 {
                self.draw_column = self.draw_area_start.0;

                self.draw_row += 8;
                if self.draw_row >= self.draw_area_end.1 {
                    self.draw_row = self.draw_area_start.1;
                }

                self.send_draw_address()?;
            }

            buffer = &buffer[count as usize..];
        }

        Ok(())
    }

    fn send_draw_address(&mut self) -> Result<(), DI::Error> {
        Command::PageAddress(self.draw_row.into()).send(&mut self.iface)?;
        Command::ColumnAddressLow(0xF & self.draw_column).send(&mut self.iface)?;
        Command::ColumnAddressHigh(0xF & (self.draw_column >> 4)).send(&mut self.iface)
    }

    /// Get the configured display size
    pub fn get_size(&self) -> DisplaySize {
        self.display_size
    }

    /// Get display dimensions, taking into account the current rotation of the display
    pub fn get_dimensions(&self) -> (u8, u8) {
        let (w, h) = self.display_size.dimensions();

        match self.display_rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => (w, h),
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => (h, w),
        }
    }

    /// Get the display rotation
    pub fn get_rotation(&self) -> DisplayRotation {
        self.display_rotation
    }

    /// Set the display rotation
    pub fn set_rotation(&mut self, display_rotation: DisplayRotation) -> Result<(), DI::Error> {
        self.display_rotation = display_rotation;

        match display_rotation {
            DisplayRotation::Rotate0 => {
                Command::SegmentRemap(true).send(&mut self.iface)?;
                Command::ReverseComDir(true).send(&mut self.iface)
            }
            DisplayRotation::Rotate90 => {
                Command::SegmentRemap(false).send(&mut self.iface)?;
                Command::ReverseComDir(true).send(&mut self.iface)
            }
            DisplayRotation::Rotate180 => {
                Command::SegmentRemap(false).send(&mut self.iface)?;
                Command::ReverseComDir(false).send(&mut self.iface)
            }
            DisplayRotation::Rotate270 => {
                Command::SegmentRemap(true).send(&mut self.iface)?;
                Command::ReverseComDir(false).send(&mut self.iface)
            }
        }
    }

    /// Set the display contrast
    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DI::Error> {
        Command::Contrast(contrast).send(&mut self.iface)
    }
}
