#![cfg_attr(not(feature = "std"), no_std)]

use embedded_hal::{
    delay::blocking::DelayUs,
    i2c::{blocking::I2c, Error as I2cError},
};

pub use config::*;
pub use error::Error;
use register::Register;

mod config;
mod error;
mod register;

pub struct Es8311<I2C> {
    i2c: I2C,
    address: Address,
}

impl<I2C: I2c<Error = E>, E: I2cError> Es8311<I2C> {
    pub fn new(i2c: I2C, address: Address) -> Self {
        Self { i2c, address }
    }

    pub fn into_inner(self) -> I2C {
        self.i2c
    }

    pub fn init<D: DelayUs>(&mut self, mut delay: D, config: &Config) -> Result<(), Error<E>> {
        self.write_reg(Register::ResetReg00, 0x1F)?;
        let _ = delay.delay_us(20);
        self.write_reg(Register::ResetReg00, 0x00)?;
        self.write_reg(Register::ResetReg00, 0x80)?;

        self.clock_config(config)?;
        self.format_config(config.res_in, config.res_out)?;

        self.write_reg(Register::SystemReg0D, 0x01)?;
        self.write_reg(Register::SystemReg0E, 0x02)?;
        self.write_reg(Register::SystemReg12, 0x00)?;
        self.write_reg(Register::SystemReg13, 0x10)?;
        self.write_reg(Register::AdcReg1C, 0x6A)?;
        self.write_reg(Register::DacReg37, 0x08)
    }

    fn clock_config(&mut self, config: &Config) -> Result<(), Error<E>> {
        let mut reg01 = 0x3F;
        let coefficients = match config.mclk {
            Some(mclk) => {
                reg01 |= 1 << 7;
                Some(mclk)
            }
            None => (config.res_in == config.res_out)
                .then_some(config.sample_frequency.as_freq() * config.res_in.bits() as u32 * 2)
                .and_then(MclkFreq::try_from_freq),
        }
        .and_then(|f| Coefficients::get(f, config.sample_frequency))
        .ok_or(Error::InvalidConfiguration)?;

        if config.mclk_inverted {
            reg01 |= 1 << 6;
        }
        self.write_reg(Register::ClkManagerReg01, reg01)?;

        let mut reg06 = self.read_reg(Register::ClkManagerReg06)?;
        if config.sclk_inverted {
            reg06 |= 1 << 5;
        } else {
            reg06 &= !(1 << 5);
        }
        self.write_reg(Register::ClkManagerReg06, reg06)?;

        self.sample_freq_config_inner(coefficients)
    }

    pub fn sample_freq_config(&mut self, mclk: MclkFreq, rate: SampleFreq) -> Result<(), Error<E>> {
        Coefficients::get(mclk, rate)
            .ok_or(Error::InvalidConfiguration)
            .and_then(|c| self.sample_freq_config_inner(c))
    }

    pub fn mic_config(&mut self, digital_mic: bool) -> Result<(), Error<E>> {
        let mut reg14 = 0x1A;
        if digital_mic {
            reg14 |= 1 << 6;
        }

        self.write_reg(Register::AdcReg17, 0xC8)?;
        self.write_reg(Register::SystemReg14, reg14)
    }

    pub fn set_voice_volume(&mut self, volume: u8) -> Result<(), Error<E>> {
        self.write_reg(Register::DacReg32, volume)
    }

    pub fn voice_volume(&mut self) -> Result<u8, Error<E>> {
        self.read_reg(Register::DacReg32)
    }

    pub fn voice_mute(&mut self, mute: bool) -> Result<(), Error<E>> {
        let mut reg31 = self.read_reg(Register::DacReg31)?;
        const MUTE: u8 = (1 << 6) | (1 << 5);
        if mute {
            reg31 |= MUTE;
        } else {
            reg31 &= !MUTE;
        }
        self.write_reg(Register::DacReg31, reg31)
    }

    pub fn set_mic_gain(&mut self, gain: Gain) -> Result<(), Error<E>> {
        self.write_reg(Register::AdcReg16, gain as u8)
    }

    pub fn set_mic_fade(&mut self, fade: Fade) -> Result<(), Error<E>> {
        self.set_fade(Register::AdcReg16, fade)
    }

    pub fn set_voice_fade(&mut self, fade: Fade) -> Result<(), Error<E>> {
        self.set_fade(Register::DacReg37, fade)
    }

    fn set_fade(&mut self, register: Register, fade: Fade) -> Result<(), Error<E>> {
        let mut reg = self.read_reg(register)?;
        reg &= 0x0F;
        reg |= (fade as u8) << 4;
        self.write_reg(register, reg)
    }

    fn sample_freq_config_inner(&mut self, coefficients: &Coefficients) -> Result<(), Error<E>> {
        let mut reg02 = self.read_reg(Register::ClkManagerReg02)?;
        reg02 &= 0x07;
        reg02 |= (coefficients.pre_div - 1) << 5;
        reg02 |= coefficients.pre_multi << 3;
        self.write_reg(Register::ClkManagerReg02, reg02)?;

        let reg03 = (coefficients.fs_mode << 6) | coefficients.adc_osr;
        self.write_reg(Register::ClkManagerReg03, reg03)?;

        self.write_reg(Register::ClkManagerReg04, coefficients.dac_osr)?;

        let reg05 = ((coefficients.adc_div - 1) << 4) | (coefficients.dac_div - 1);
        self.write_reg(Register::ClkManagerReg05, reg05)?;

        let mut reg06 = self.read_reg(Register::ClkManagerReg06)?;
        reg06 &= 0xE0;
        reg06 |= coefficients.bclk_div - (coefficients.bclk_div < 19) as u8;
        self.write_reg(Register::ClkManagerReg06, reg06)?;

        let mut reg07 = self.read_reg(Register::ClkManagerReg07)?;
        reg07 &= 0xC0;
        reg07 |= coefficients.lrck_h;
        self.write_reg(Register::ClkManagerReg07, reg07)?;
        self.write_reg(Register::ClkManagerReg08, coefficients.lrck_l)
    }

    fn format_config(&mut self, res_in: Resolution, res_out: Resolution) -> Result<(), Error<E>> {
        let reg00 = self.read_reg(Register::ResetReg00)?;
        self.write_reg(Register::ResetReg00, reg00 & 0xBF)?;
        self.write_reg(Register::SdpInReg09, res_in.config())?;
        self.write_reg(Register::SdpOutReg0A, res_out.config())
    }

    pub fn dump_regs(&mut self) -> Result<(), Error<E>> {
        use strum::IntoEnumIterator;
        for register in Register::iter() {
            let reg_val = register as u8;
            let val = self.read_reg(register)?;
            log::info!("register {register:?} at address {reg_val:#02X} with value {val:#02X}")
        }
        Ok(())
    }

    fn read_reg(&mut self, reg: Register) -> Result<u8, Error<E>> {
        use core::array::from_mut;
        let mut value = 0;
        self.i2c
            .write_read(self.address as u8, &[reg as u8], from_mut(&mut value))
            .map_err(Error::BusError)?;
        Ok(value)
    }

    fn write_reg(&mut self, reg: Register, value: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address as u8, &[reg as u8, value])
            .map_err(Error::BusError)
    }
}
