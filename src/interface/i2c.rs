//! sh1106 I2C Interface

use hal;

use super::DisplayInterface;

// TODO: Add to prelude
/// sh1106 I2C communication interface
pub struct I2cInterface<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C> I2cInterface<I2C>
where
    I2C: hal::blocking::i2c::Write,
{
    /// Create new sh1106 I2C interface
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Self { i2c, addr }
    }
}

impl<I2C> DisplayInterface for I2cInterface<I2C>
where
    I2C: hal::blocking::i2c::Write,
{
    fn send_commands(&mut self, cmds: &[u8]) -> Result<(), ()> {
        // Copy over given commands to new aray to prefix with command identifier
        let mut writebuf: [u8; 8] = [0; 8];
        writebuf[1..=cmds.len()].copy_from_slice(&cmds);

        self.i2c
            .write(self.addr, &writebuf[..=cmds.len()])
            .map_err(|_| ())?;

        Ok(())
    }

    fn send_data(&mut self, buf: &[u8]) -> Result<(), ()> {
        const CHUNKLEN: usize = 128;

        // 4 control bytes
        const BUFLEN: usize = CHUNKLEN + 1;

        // TODO: Use screen width var instead of const

        // Noop if the data buffer is empty
        if buf.is_empty() {
            return Ok(());
        }

        // TODO: Use commands. Page command starts at 0xb0, gets incremented for each page
        let mut page = 0xb0;

        // Display width plus 4 start bytes
        let mut writebuf: [u8; BUFLEN] = [0; BUFLEN];

        writebuf[0] = 0x40; // Data
                            // writebuf[1] = page; // Page address
                            // writebuf[2] = 0x02; // Lower column address
                            // writebuf[3] = 0x10; // Upper column address (always zero, base is 10h)

        for chunk in buf.chunks(CHUNKLEN) {
            writebuf[1] = page;

            // Copy over all data from buffer, leaving the data command bytes intact
            writebuf[1..BUFLEN].copy_from_slice(&chunk);

            self.i2c
                .write(
                    self.addr,
                    &[
                        0x00, // Command
                        page, // Page address
                        0x02, // Lower column address
                        0x10, // Upper column address (always zero, base is 10h)
                    ],
                )
                .map_err(|_| ())?;

            self.i2c.write(self.addr, &writebuf).map_err(|_| ())?;

            page += 1;
        }

        Ok(())
    }
}
