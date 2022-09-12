pub(crate) use coefficients::Coefficients;

mod coefficients;

#[derive(Copy, Clone, Debug)]
pub enum SampleFreq {
    Freq8KHz,
    Freq11KHz,
    Freq12KHz,
    Freq16KHz,
    Freq22KHz,
    Freq24KHz,
    Freq32KHz,
    Freq44KHz,
    Freq48KHz,
    Freq64KHz,
    Freq88KHz,
    Freq96KHz,
}

impl SampleFreq {
    pub const fn as_freq(self) -> u32 {
        use SampleFreq::*;
        match self {
            Freq8KHz => 8_000,
            Freq11KHz => 11_025,
            Freq12KHz => 12_000,
            Freq16KHz => 16_000,
            Freq22KHz => 22_050,
            Freq24KHz => 24_000,
            Freq32KHz => 32_000,
            Freq44KHz => 44_100,
            Freq48KHz => 48_000,
            Freq64KHz => 64_000,
            Freq88KHz => 88_200,
            Freq96KHz => 96_000,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MclkFreq {
    Freq706KHz,
    Freq1024KHz,
    Freq1411KHz,
    Freq1536KHz,
    Freq2048KHz,
    Freq2822KHz,
    Freq3072KHz,
    Freq4096KHz,
    Freq5645KHz,
    Freq6144KHz,
    Freq8192KHz,
    Freq11Mhz,
    Freq12Mhz,
    Freq16Mhz,
    Freq18Mhz,
}

impl MclkFreq {
    pub const fn as_freq(self) -> u32 {
        use MclkFreq::*;
        match self {
            Freq706KHz => 705_600,
            Freq1024KHz => 1_024_000,
            Freq1411KHz => 1_411_200,
            Freq1536KHz => 1_536_000,
            Freq2048KHz => 2_048_000,
            Freq2822KHz => 2_822_400,
            Freq3072KHz => 3_072_000,
            Freq4096KHz => 4_096_000,
            Freq5645KHz => 5_644_800,
            Freq6144KHz => 6_144_000,
            Freq8192KHz => 8_192_000,
            Freq11Mhz => 11_289_600,
            Freq12Mhz => 12_288_000,
            Freq16Mhz => 16_384_000,
            Freq18Mhz => 18_432_000,
        }
    }

    pub const fn try_from_freq(freq: u32) -> Option<Self> {
        use MclkFreq::*;
        match freq {
            705_600 => Some(Freq706KHz),
            1_024_000 => Some(Freq1024KHz),
            1_411_200 => Some(Freq1411KHz),
            1_536_000 => Some(Freq1536KHz),
            2_048_000 => Some(Freq2048KHz),
            2_822_400 => Some(Freq2822KHz),
            3_072_000 => Some(Freq3072KHz),
            4_096_000 => Some(Freq4096KHz),
            5_644_800 => Some(Freq5645KHz),
            6_144_000 => Some(Freq6144KHz),
            8_192_000 => Some(Freq8192KHz),
            11_289_600 => Some(Freq11Mhz),
            12_288_000 => Some(Freq12Mhz),
            16_384_000 => Some(Freq16Mhz),
            18_432_000 => Some(Freq18Mhz),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Gain {
    GainMin = -1,
    Gain0db,
    Gain6db,
    Gain12db,
    Gain18db,
    Gain24db,
    Gain30db,
    Gain36db,
    Gain42db,
    GainMax,
}

#[derive(Copy, Clone, Debug)]
pub enum Fade {
    FadeOff = 0,
    Fade4LRCK, // 4LRCK means ramp 0.25dB/4LRCK
    Fade8LRCK,
    Fade16LRCK,
    Fade32LRCK,
    Fade64LRCK,
    Fade128LRCK,
    Fade256LRCK,
    Fade512LRCK,
    Fade1024LRCK,
    Fade2048LRCK,
    Fade4096LRCK,
    Fade8192LRCK,
    Fade16384LRCK,
    Fade32768LRCK,
    Fade65536LRCK,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Resolution {
    Resolution16 = 3,
    Resolution18 = 2,
    Resolution20 = 1,
    Resolution24 = 0,
    Resolution32 = 4,
}

impl Resolution {
    pub(crate) const fn bits(self) -> u8 {
        use Resolution::*;
        match self {
            Resolution16 => 16,
            Resolution18 => 18,
            Resolution20 => 20,
            Resolution24 => 24,
            Resolution32 => 32,
        }
    }

    pub(crate) const fn config(self) -> u8 {
        (self as u8) << 2
    }
}

pub struct Config {
    pub sample_frequency: SampleFreq,
    pub mclk: Option<MclkFreq>,
    pub res_in: Resolution,
    pub res_out: Resolution,
    pub mclk_inverted: bool,
    pub sclk_inverted: bool,
}

/// I²C slave addresses, determined by the logic level of pin `CE`
#[derive(Copy, Clone)]
pub enum Address {
    /// `CE` pin == 0
    Primary = 0x18,
    /// `CE` pin == 1
    Secondary = 0x19,
}
