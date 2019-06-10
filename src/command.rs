use super::interface::DisplayInterface;

/// sh1106 Commands

/// Commands
#[derive(Debug)]
#[allow(dead_code)]
pub enum Command {
    /// Set contrast. Higher number is higher contrast. Default = 0x7F
    Contrast(u8),
    /// Turn entire display on. If set, all pixels will
    /// be set to on, if not, the value in memory will be used.
    AllOn(bool),
    /// Invert display.
    Invert(bool),
    /// Turn display on or off.
    DisplayOn(bool),
    /// Set column address lower 4 bits
    ColumnAddressLow(u8),
    /// Set column address higher 4 bits
    ColumnAddressHigh(u8),
    /// Set page address
    PageAddress(Page),
    /// Set display start line from 0-63
    StartLine(u8),
    /// Reverse columns from 127-0
    SegmentRemap(bool),
    /// Set multipex ratio from 15-63 (MUX-1)
    Multiplex(u8),
    /// Scan from COM[n-1] to COM0 (where N is mux ratio)
    ReverseComDir(bool),
    /// Set vertical shift
    DisplayOffset(u8),
    /// Setup com hardware configuration
    /// First value indicates sequential (false) or alternative (true)
    /// pin configuration.
    ComPinConfig(bool),
    /// Set up display clock.
    /// First value is oscillator frequency, increasing with higher value
    /// Second value is divide ratio - 1
    DisplayClockDiv(u8, u8),
    /// Set up phase 1 and 2 of precharge period. each value is from 0-63
    PreChargePeriod(u8, u8),
    /// Set Vcomh Deselect level
    VcomhDeselect(VcomhLevel),
    /// NOOP
    Noop,
    /// Enable charge pump
    ChargePump(bool),
}

impl Command {
    /// Send command to sh1106
    pub fn send<DI>(self, iface: &mut DI) -> Result<(), DI::Error>
    where
        DI: DisplayInterface,
    {
        // Transform command into a fixed size array of 7 u8 and the real length for sending
        let (data, len) = match self {
            Command::Contrast(val) => ([0x81, val, 0, 0, 0, 0, 0], 2),
            Command::AllOn(on) => ([0xA4 | (on as u8), 0, 0, 0, 0, 0, 0], 1),
            Command::Invert(inv) => ([0xA6 | (inv as u8), 0, 0, 0, 0, 0, 0], 1),
            Command::DisplayOn(on) => ([0xAE | (on as u8), 0, 0, 0, 0, 0, 0], 1),
            Command::ColumnAddressLow(addr) => ([0xF & addr, 0, 0, 0, 0, 0, 0], 1),
            Command::ColumnAddressHigh(addr) => ([0x10 | (0xF & addr), 0, 0, 0, 0, 0, 0], 1),
            Command::PageAddress(page) => ([0xB0 | (page as u8), 0, 0, 0, 0, 0, 0], 1),
            Command::StartLine(line) => ([0x40 | (0x3F & line), 0, 0, 0, 0, 0, 0], 1),
            Command::SegmentRemap(remap) => ([0xA0 | (remap as u8), 0, 0, 0, 0, 0, 0], 1),
            Command::Multiplex(ratio) => ([0xA8, ratio, 0, 0, 0, 0, 0], 2),
            Command::ReverseComDir(rev) => ([0xC0 | ((rev as u8) << 3), 0, 0, 0, 0, 0, 0], 1),
            Command::DisplayOffset(offset) => ([0xD3, offset, 0, 0, 0, 0, 0], 2),
            Command::ComPinConfig(alt) => ([0xDA, 0x02 | ((alt as u8) << 4), 0, 0, 0, 0, 0], 2),
            Command::DisplayClockDiv(fosc, div) => {
                ([0xD5, ((0xF & fosc) << 4) | (0xF & div), 0, 0, 0, 0, 0], 2)
            }
            Command::PreChargePeriod(phase1, phase2) => (
                [0x22, ((0xF & phase2) << 4) | (0xF & phase1), 0, 0, 0, 0, 0],
                2,
            ),
            Command::VcomhDeselect(level) => ([0x35, (level as u8) << 4, 0, 0, 0, 0, 0], 2),
            Command::Noop => ([0xE3, 0, 0, 0, 0, 0, 0], 1),
            Command::ChargePump(en) => ([0xAD, 0x8A | (en as u8), 0, 0, 0, 0, 0], 2),
        };

        // Send command over the interface
        iface.send_commands(&data[0..len])
    }
}

/// Display page
#[derive(Debug, Clone, Copy)]
pub enum Page {
    /// Page 0
    Page0 = 0,
    /// Page 1
    Page1 = 1,
    /// Page 2
    Page2 = 2,
    /// Page 3
    Page3 = 3,
    /// Page 4
    Page4 = 4,
    /// Page 5
    Page5 = 5,
    /// Page 6
    Page6 = 6,
    /// Page 7
    Page7 = 7,
}

impl From<u8> for Page {
    fn from(val: u8) -> Page {
        match val / 8 {
            0 => Page::Page0,
            1 => Page::Page1,
            2 => Page::Page2,
            3 => Page::Page3,
            4 => Page::Page4,
            5 => Page::Page5,
            6 => Page::Page6,
            7 => Page::Page7,
            _ => panic!("Page too high"),
        }
    }
}

/// Frame interval
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum NFrames {
    /// 2 Frames
    F2 = 0b111,
    /// 3 Frames
    F3 = 0b100,
    /// 4 Frames
    F4 = 0b101,
    /// 5 Frames
    F5 = 0b000,
    /// 25 Frames
    F25 = 0b110,
    /// 64 Frames
    F64 = 0b001,
    /// 128 Frames
    F128 = 0b010,
    /// 256 Frames
    F256 = 0b011,
}

/// Vcomh Deselect level
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum VcomhLevel {
    /// 0.65 * Vcc
    V065 = 0b001,
    /// 0.77 * Vcc
    V077 = 0b010,
    /// 0.83 * Vcc
    V083 = 0b011,
    /// Auto
    Auto = 0b100,
}
