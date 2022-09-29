use proc_bitfield::bitfield;

macro_rules! register_group {
    ($group:ident, $(($name:ident, $address:literal, $default:literal)),+$(,)?) => {
        $(register!($name, $address, $default);)+

        #[derive(Debug)]
        pub(crate) struct $group($($crate::register::WrappedRegister<$name>,)+);

        impl $group {
            pub(crate) fn read_group<I2C: $crate::I2c<Error = E>, E>(
                es8311: &mut $crate::Es8311LowLevel<I2C>
            ) -> Result<Self, E> {
                Ok(Self($($crate::register::WrappedRegister(es8311.read_register::<$name>()?),)+))
            }
        }
    };
}

macro_rules! register {
    ($name:ident, $address:literal, $default:literal) => {
        impl $name {
            #[must_use]
            pub const fn new() -> Self {
                Self(0x00)
            }
        }

        impl ::core::convert::From<u8> for $name {
            fn from(val: u8) -> Self {
                Self(val)
            }
        }

        impl ::core::convert::From<$name> for u8 {
            fn from($name(val): $name) -> Self {
                val
            }
        }

        impl ::core::default::Default for $name {
            fn default() -> Self {
                Self($default)
            }
        }

        impl $crate::register::private::Sealed for $name {}

        impl $crate::register::Register for $name {
            const ADDRESS: u8 = $address;
        }
    };
}

pub trait Register: From<u8> + Into<u8> + Default + private::Sealed {
    const ADDRESS: u8;
}

mod private {
    pub trait Sealed {}
}

struct WrappedRegister<R>(R);

impl<R: Register + core::fmt::Debug> core::fmt::Debug for WrappedRegister<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut debug = f.debug_struct("Register");
        debug.field("address", &R::ADDRESS);
        debug.field("register", &self.0).finish()
    }
}

pub use {adc::*, adc_eq::*, clock_manager::*, dac::*, dac_eq::*, system::*};

mod adc;
mod adc_eq;
mod clock_manager;
mod dac;
mod dac_eq;
mod system;

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Reset(u8): Debug {
        pub csm_on: bool @ 7,
        pub msc: bool @ 6,
        pub seq_dis: bool @ 5,
        pub rst_dig: bool @ 4,
        pub rst_cmg: bool @ 3,
        pub rst_mst: bool @ 2,
        pub rst_adc_dig: bool @ 1,
        pub rst_dac_dig: bool @ 0,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct SdpIn(u8): Debug {
        pub sdp_in_sel: bool @ 7,
        pub sdp_in_mute: bool @ 6,
        pub sdp_in_lrp: bool @ 5,
        pub sdp_in_wl: u8 @ 2..=4,
        pub sdp_in_fmt: u8 @ 0..=1,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct SdpOut(u8): Debug {
        pub sdp_out_mute: bool @ 6,
        pub sdp_out_lrp: bool @ 5,
        pub sdp_out_wl: u8 @ 2..=4,
        pub sdp_out_fmt: u8 @ 0..=1,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Gpio(u8): Debug {
        pub adc2dac_sel: bool @ 7,
        pub adcdat_sel: u8 @ 4..=6,
        pub i2c_wl: bool @ 3,
        pub gpio_sel: u8 [ro] @ 0..=2,

    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Gp(u8): Debug {
        pub forcecsm: u8 [ro] @ 4..=7,
        pub adc_dly_sel: bool [ro] @ 0,
        pub dac_dly_sel: bool [ro] @ 0,
        pub dac_autochn: bool [ro] @ 0,
        pub pullup_se: bool @ 0,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct I2c(u8): Debug {
        pub i2c_retime: bool [ro] @ 1,
        pub ini_reg: bool @ 0,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Flag(u8): Debug {
        pub flag_csm_chip: u8 [ro] @ 4..=6,
        pub flag_adcam: bool [ro] @ 1,
        pub flag_dacam: bool [ro] @ 0,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ChipId1(u8): Debug {
        pub chip_id1: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ChipId2(u8): Debug {
        pub chip_id2: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ChipVer(u8): Debug {
        pub chip_ver: u8 @ 0..=7,
    }
}

pub(crate) fn is_es8311(id1: ChipId1, id2: ChipId2) -> bool {
    id1 == Default::default() && id2 == Default::default()
}

register!(Reset, 0x00, 0b0001_1111);
register!(SdpIn, 0x09, 0b0000_0000);
register!(SdpOut, 0x0A, 0b0000_0000);
register!(Gpio, 0x44, 0b0000_0000);
register!(Gp, 0x045, 0b0000_0000);
register!(I2c, 0x0FA, 0b0000_0000);
register!(Flag, 0x0FC, 0b0000_0000);
register!(ChipId1, 0xFD, 0x83);
register!(ChipId2, 0xFE, 0x11);
register!(ChipVer, 0xFF, 0b0000_0000);

#[derive(Debug)]
pub(crate) struct RegisterDump(
    WrappedRegister<Reset>,
    ClockManager,
    WrappedRegister<SdpIn>,
    WrappedRegister<SdpOut>,
    System,
    Adc,
    AdcEqualizer,
    Dac,
    DacEqualizer,
    WrappedRegister<Gpio>,
    WrappedRegister<Gp>,
    WrappedRegister<I2c>,
    WrappedRegister<Flag>,
    WrappedRegister<ChipId1>,
    WrappedRegister<ChipId2>,
    WrappedRegister<ChipVer>,
);

impl RegisterDump {
    pub(crate) fn read_group<I2C: super::I2c<Error = E>, E>(
        es8311: &mut super::Es8311LowLevel<I2C>,
    ) -> Result<Self, E> {
        Ok(Self(
            WrappedRegister(es8311.read_register()?),
            ClockManager::read_group(es8311)?,
            WrappedRegister(es8311.read_register()?),
            WrappedRegister(es8311.read_register()?),
            System::read_group(es8311)?,
            Adc::read_group(es8311)?,
            AdcEqualizer::read_group(es8311)?,
            Dac::read_group(es8311)?,
            DacEqualizer::read_group(es8311)?,
            WrappedRegister(es8311.read_register()?),
            WrappedRegister(es8311.read_register()?),
            WrappedRegister(es8311.read_register()?),
            WrappedRegister(es8311.read_register()?),
            WrappedRegister(es8311.read_register()?),
            WrappedRegister(es8311.read_register()?),
            WrappedRegister(es8311.read_register()?),
        ))
    }
}
