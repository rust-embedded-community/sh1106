//! Helpers for use in examples and tests

use embedded_hal::{
    i2c::{self, Operation},
    spi,
    digital::OutputPin,
};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct SpiStub;

impl hal::spi::ErrorType for SpiStub {
    type Error = core::convert::Infallible;
}
impl spi::SpiBus<u8> for SpiStub {
    fn write(&mut self, _buf: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn read(&mut self, _buf: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn transfer(&mut self, _buf: &mut [u8], _buf2: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn transfer_in_place(&mut self, _buf: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct PinStub;
impl hal::digital::ErrorType for PinStub {
    type Error = core::convert::Infallible;
}
impl OutputPin for PinStub {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct I2cStub;

impl hal::i2c::ErrorType for I2cStub {
    type Error = core::convert::Infallible;
}
impl i2c::I2c for I2cStub {
    fn transaction(&mut self, _addr: u8, _buf: &mut [Operation<'_>]) -> Result<(), Self::Error> {
        Ok(())
    }
}
