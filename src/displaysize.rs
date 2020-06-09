//! Display size

/// Display size enumeration
#[derive(Clone, Copy)]
pub enum DisplaySize {
    /// 128 by 64 pixels
    Display128x64,
    /// 128 by 64 pixels without 2px X offset
    Display128x64NoOffset,
    /// 128 by 32 pixels
    Display128x32,
    /// 132 by 64 pixels
    Display132x64,
}

impl DisplaySize {
    /// Get integral dimensions from DisplaySize
    pub fn dimensions(self) -> (u8, u8) {
        match self {
            DisplaySize::Display128x64 => (128, 64),
            DisplaySize::Display128x64NoOffset => (128, 64),
            DisplaySize::Display128x32 => (128, 32),
            DisplaySize::Display132x64 => (132, 64),
        }
    }

    /// Get the panel column offset from DisplaySize
    pub fn column_offset(self) -> u8 {
        match self {
            DisplaySize::Display128x64 => 2,
            DisplaySize::Display128x64NoOffset => 0,
            DisplaySize::Display128x32 => 2,
            DisplaySize::Display132x64 => 0,
        }
    }
}
