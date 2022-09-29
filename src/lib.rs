#![cfg_attr(not(feature = "std"), no_std)]

use embedded_hal::{
    delay::blocking::DelayUs,
    i2c::{blocking::I2c, Error as I2cError},
};

pub use config::*;
pub use error::Error;
use register::*;

mod config;
mod error;
mod register;

pub struct Es8311LowLevel<I2C> {
    i2c: I2C,
    address: Address,
}

impl<I2C: I2c<Error = E>, E: I2cError> Es8311LowLevel<I2C> {
    pub fn new(i2c: I2C, address: Address) -> Self {
        Self { i2c, address }
    }

    pub fn into_inner(self) -> I2C {
        self.i2c
    }

    #[inline]
    pub fn read_register<R: Register>(&mut self) -> Result<R, E> {
        let mut value = [0];
        self.i2c
            .write_read(self.address as u8, &[R::ADDRESS], &mut value)?;
        Ok(value[0].into())
    }

    #[inline]
    pub fn write_register<R: Register>(&mut self, reg: R) -> Result<(), E> {
        self.i2c
            .write(self.address as u8, &[R::ADDRESS, reg.into()])
    }

    #[inline]
    pub fn update_reg<R: Register, F: FnOnce(R) -> R>(&mut self, f: F) -> Result<(), E> {
        self.read_register()
            .and_then(|reg| self.write_register(f(reg)))
    }
}

pub struct Es8311<I2C> {
    i2c: I2C,
    address: Address,
}

impl<I2C: I2c<Error = E>, E: I2cError> Es8311<I2C> {
    pub fn new(i2c: I2C, address: Address) -> Self {
        Self { i2c, address }
    }

    // fn dump_reg<R: Register + Debug>(&mut self) -> Result<impl Debug + Sized, Error<E>> {
    //     let reg = self.read_reg::<R>()?;
    //     log::info!("{reg:#02X?} addr: {}", R::ADDRESS);
    //     Ok(())
    // }

    pub fn dump_regs(&mut self) -> Result<impl core::fmt::Debug + Sized, Error<E>> {
        RegisterDump::read_group(self)
    }

    pub fn into_inner(self) -> I2C {
        self.i2c
    }

    pub fn init<D: DelayUs>(&mut self, mut delay: D, config: &Config) -> Result<(), Error<E>> {
        self.write_reg(
            Reset::new()
                .with_rst_dig(true)
                .with_rst_cmg(true)
                .with_rst_mst(true)
                .with_rst_adc_dig(true)
                .with_rst_dac_dig(true),
        )?;
        let _ = delay.delay_us(20);
        self.write_reg(Reset::new())?;
        self.write_reg(Reset::new().with_csm_on(true))?;

        self.clock_config(config)?;
        self.format_config(config.res_in, config.res_out)?;

        self.write_reg(System03::new().with_vmidsel(0x02))?;
        self.write_reg(System04::new())?;
        self.write_reg(System08::new())?;
        self.write_reg(System09::new().with_hpsw(true))?;
        self.write_reg(
            Adc08::new()
                .with_adc_eqbypass(true)
                .with_adc_hpf(true)
                .with_adc_hpfs2(0x0A),
        )?;
        self.write_reg(Dac07::new().with_dac_eqbypass(true))
    }

    fn clock_config(&mut self, config: &Config) -> Result<(), Error<E>> {
        let mut mclk_sel = true;
        let coefficients = config
            .mclk
            .or_else(|| {
                mclk_sel = false;
                (config.res_in == config.res_out)
                    .then_some(config.sample_frequency.as_freq() * config.res_in.bits() as u32 * 2)
                    .and_then(MclkFreq::try_from_freq)
            })
            .and_then(|f| Coefficients::get(f, config.sample_frequency))
            .ok_or(Error::InvalidConfiguration)?;

        self.write_reg(
            ClockManager01::new()
                .with_mclk_on(true)
                .with_bclk_on(true)
                .with_clkadc_on(true)
                .with_clkdac_on(true)
                .with_anaclkadc_on(true)
                .with_anaclkdac_on(true)
                .with_mclk_inv(config.mclk_inverted)
                .with_mclk_sel(mclk_sel),
        )?;

        self.update_reg(|reg: ClockManager06| reg.with_bclk_inv(config.sclk_inverted))?;

        self.sample_freq_config_inner(coefficients)
    }

    pub fn sample_freq_config(&mut self, mclk: MclkFreq, rate: SampleFreq) -> Result<(), Error<E>> {
        Coefficients::get(mclk, rate)
            .ok_or(Error::InvalidConfiguration)
            .and_then(|c| self.sample_freq_config_inner(c))
    }

    pub fn mic_config(&mut self, digital_mic: bool) -> Result<(), Error<E>> {
        self.write_reg(Adc03::new().with_adc_volume(0xC8))?;
        self.write_reg(
            System10::new()
                .with_linsel(true)
                .with_dmic_on(digital_mic)
                .with_pgagain(0x0A),
        )
    }

    pub fn set_voice_volume(&mut self, volume: u8) -> Result<(), Error<E>> {
        self.write_reg(Dac02::new().with_dac_volume(volume))
    }

    pub fn voice_volume(&mut self) -> Result<u8, Error<E>> {
        self.read_reg().map(|reg: Dac02| reg.dac_volume())
    }

    pub fn voice_mute(&mut self, mute: bool) -> Result<(), Error<E>> {
        self.update_reg(|reg: Dac01| reg.with_dac_dsmmute(mute).with_dac_demmute(mute))
    }

    pub fn set_mic_gain(&mut self, gain: Gain) -> Result<(), Error<E>> {
        self.write_reg(Adc02::new().with_adc_scale(gain as u8))
    }

    pub fn set_mic_fade(&mut self, fade: Fade) -> Result<(), Error<E>> {
        self.update_reg(|reg: Adc01| reg.with_adc_ramprate(fade as u8))
    }

    pub fn set_voice_fade(&mut self, fade: Fade) -> Result<(), Error<E>> {
        self.update_reg(|reg: Dac07| reg.with_dac_ramprate(fade as u8))
    }

    fn sample_freq_config_inner(&mut self, coefficients: &Coefficients) -> Result<(), Error<E>> {
        self.update_reg(|reg: ClockManager02| {
            reg.with_div_pre(coefficients.pre_div - 1)
                .with_mult_pre(coefficients.pre_multi)
        })?;

        self.write_reg(
            ClockManager03::new()
                .with_adc_fsmode(coefficients.fs_mode)
                .with_adc_osr(coefficients.adc_osr),
        )?;

        self.write_reg(ClockManager04::new().with_dac_osr(coefficients.dac_osr))?;

        self.write_reg(
            ClockManager05::new()
                .with_div_clkadc(coefficients.adc_div - 1)
                .with_div_clkdac(coefficients.dac_div - 1),
        )?;

        self.update_reg(|reg: ClockManager06| {
            reg.with_div_bclk(coefficients.bclk_div - (coefficients.bclk_div < 19) as u8)
        })?;

        self.write_reg(ClockManager07::new().with_div_lrck_8_11(coefficients.lrck_h))?;
        self.write_reg(ClockManager08::new().with_div_lrck_0_7(coefficients.lrck_l))
    }

    fn format_config(&mut self, res_in: Resolution, res_out: Resolution) -> Result<(), Error<E>> {
        self.update_reg(|reg: Reset| reg.with_msc(false))?;
        self.write_reg(SdpIn::new().with_sdp_in_wl(res_in as u8))?;
        self.write_reg(SdpOut::new().with_sdp_out_wl(res_out as u8))
    }
    //
    // pub fn dump_regs(&mut self) -> Result<(), Error<E>> {
    //     use strum::IntoEnumIterator;
    //     for register in RegisterE::iter() {
    //         let reg_val = register as u8;
    //         let val = self.read_reg(register)?;
    //         log::info!("register {register:?} at address {reg_val:#02X} with value {val:#02X}")
    //     }
    //     Ok(())
    // }

    #[inline]
    fn read_reg<R: Register>(&mut self) -> Result<R, Error<E>> {
        let mut value = [0];
        self.i2c
            .write_read(self.address as u8, &[R::ADDRESS], &mut value)
            .map_err(Error::BusError)?;
        Ok(value[0].into())
    }

    #[inline]
    fn write_reg<R: Register>(&mut self, reg: R) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address as u8, &[R::ADDRESS, reg.into()])
            .map_err(Error::BusError)
    }

    #[inline]
    fn update_reg<R: Register, F: FnOnce(R) -> R>(&mut self, f: F) -> Result<(), Error<E>> {
        self.read_reg().and_then(|reg| self.write_reg(f(reg)))
    }
}
