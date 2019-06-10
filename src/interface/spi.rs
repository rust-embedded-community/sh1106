//! sh1106 SPI interface

use hal;
use hal::digital::OutputPin;

use super::DisplayInterface;
use crate::Error;

/// SPI display interface.
///
/// This combines the SPI peripheral and a data/command pin
pub struct SpiInterface<SPI, DC, CS> {
    spi: SPI,
    dc: DC,
    cs: CS,
}

impl<SPI, DC, CS, CommE> SpiInterface<SPI, DC, CS>
where
    SPI: hal::blocking::spi::Write<u8, Error = CommE>,
    DC: OutputPin,
    CS: OutputPin,
{
    /// Create new SPI interface for communciation with sh1106
    pub fn new(spi: SPI, dc: DC, mut cs: CS) -> Self {
        cs.set_high();

        Self { spi, dc, cs }
    }
}

impl<SPI, DC, CS, CommE> DisplayInterface for SpiInterface<SPI, DC, CS>
where
    SPI: hal::blocking::spi::Write<u8, Error = CommE>,
    DC: OutputPin,
    CS: OutputPin,
{
    type Error = Error<CommE>;

    fn send_commands(&mut self, cmds: &[u8]) -> Result<(), Self::Error> {
        self.cs.set_low();
        self.dc.set_low();

        self.spi.write(&cmds).map_err(Error::Comm)?;

        self.dc.set_high();
        self.cs.set_high();

        Ok(())
    }

    fn send_data(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.cs.set_low();

        // 1 = data, 0 = command
        self.dc.set_high();

        self.spi.write(&buf).map_err(Error::Comm)?;

        self.cs.set_high();

        Ok(())
    }
}
