use super::{MclkFreq, SampleFreq};

#[derive(Copy, Clone)]
pub(crate) struct Coefficients {
    pub mclk: MclkFreq,
    pub rate: SampleFreq,
    pub pre_div: u8,
    pub pre_multi: u8,
    pub adc_div: u8,
    pub dac_div: u8,
    pub fs_mode: u8,
    pub lrck_h: u8,
    pub lrck_l: u8,
    pub bclk_div: u8,
    pub adc_osr: u8,
    pub dac_osr: u8,
}

impl Coefficients {
    pub const fn get(mclk: MclkFreq, rate: SampleFreq) -> Option<&'static Self> {
        const COEFF_MAP: [u8; 256] = {
            let mut arr = [COEFFICIENTS.len() as u8; 256];
            let mut i = 0;

            while i < COEFFICIENTS.len() {
                let Coefficients { mclk, rate, .. } = COEFFICIENTS[i];
                arr[index(mclk, rate)] = i as u8;
                i += 1;
            }
            arr
        };
        let idx = COEFF_MAP[index(mclk, rate)] as usize;
        if idx < COEFFICIENTS.len() {
            Some(&COEFFICIENTS[idx])
        } else {
            None
        }
    }
}

const fn index(mclk: MclkFreq, rate: SampleFreq) -> usize {
    (((mclk as u8) << 4) | (rate as u8)) as usize
}

const COEFFICIENTS: [Coefficients; 76] = [
    Coefficients {
        mclk: MclkFreq::Freq12Mhz,
        rate: SampleFreq::Freq8KHz,
        pre_div: 0x06,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq18Mhz,
        rate: SampleFreq::Freq8KHz,
        pre_div: 0x03,
        pre_multi: 0x01,
        adc_div: 0x03,
        dac_div: 0x03,
        fs_mode: 0x00,
        lrck_h: 0x05,
        lrck_l: 0xff,
        bclk_div: 0x18,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq16Mhz,
        rate: SampleFreq::Freq8KHz,
        pre_div: 0x08,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq8192KHz,
        rate: SampleFreq::Freq8KHz,
        pre_div: 0x04,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq6144KHz,
        rate: SampleFreq::Freq8KHz,
        pre_div: 0x03,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq4096KHz,
        rate: SampleFreq::Freq8KHz,
        pre_div: 0x02,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq3072KHz,
        rate: SampleFreq::Freq8KHz,
        pre_div: 0x01,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq2048KHz,
        rate: SampleFreq::Freq8KHz,
        pre_div: 0x01,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq1536KHz,
        rate: SampleFreq::Freq8KHz,
        pre_div: 0x03,
        pre_multi: 0x02,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq1024KHz,
        rate: SampleFreq::Freq8KHz,
        pre_div: 0x01,
        pre_multi: 0x01,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq11Mhz,
        rate: SampleFreq::Freq11KHz,
        pre_div: 0x04,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq5645KHz,
        rate: SampleFreq::Freq11KHz,
        pre_div: 0x02,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq2822KHz,
        rate: SampleFreq::Freq11KHz,
        pre_div: 0x01,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq1411KHz,
        rate: SampleFreq::Freq11KHz,
        pre_div: 0x01,
        pre_multi: 0x01,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq12Mhz,
        rate: SampleFreq::Freq12KHz,
        pre_div: 0x04,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq6144KHz,
        rate: SampleFreq::Freq12KHz,
        pre_div: 0x02,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq3072KHz,
        rate: SampleFreq::Freq12KHz,
        pre_div: 0x01,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq1536KHz,
        rate: SampleFreq::Freq12KHz,
        pre_div: 0x01,
        pre_multi: 0x01,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq12Mhz,
        rate: SampleFreq::Freq16KHz,
        pre_div: 0x03,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq18Mhz,
        rate: SampleFreq::Freq16KHz,
        pre_div: 0x03,
        pre_multi: 0x01,
        adc_div: 0x03,
        dac_div: 0x03,
        fs_mode: 0x00,
        lrck_h: 0x02,
        lrck_l: 0xff,
        bclk_div: 0x0c,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq16Mhz,
        rate: SampleFreq::Freq16KHz,
        pre_div: 0x04,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq8192KHz,
        rate: SampleFreq::Freq16KHz,
        pre_div: 0x02,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq6144KHz,
        rate: SampleFreq::Freq16KHz,
        pre_div: 0x03,
        pre_multi: 0x01,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq4096KHz,
        rate: SampleFreq::Freq16KHz,
        pre_div: 0x01,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq3072KHz,
        rate: SampleFreq::Freq16KHz,
        pre_div: 0x03,
        pre_multi: 0x02,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq2048KHz,
        rate: SampleFreq::Freq16KHz,
        pre_div: 0x01,
        pre_multi: 0x01,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq1536KHz,
        rate: SampleFreq::Freq16KHz,
        pre_div: 0x03,
        pre_multi: 0x03,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq1024KHz,
        rate: SampleFreq::Freq16KHz,
        pre_div: 0x01,
        pre_multi: 0x02,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq11Mhz,
        rate: SampleFreq::Freq22KHz,
        pre_div: 0x02,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq5645KHz,
        rate: SampleFreq::Freq22KHz,
        pre_div: 0x01,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq2822KHz,
        rate: SampleFreq::Freq22KHz,
        pre_div: 0x01,
        pre_multi: 0x01,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq1411KHz,
        rate: SampleFreq::Freq22KHz,
        pre_div: 0x01,
        pre_multi: 0x02,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq706KHz,
        rate: SampleFreq::Freq22KHz,
        pre_div: 0x01,
        pre_multi: 0x03,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq12Mhz,
        rate: SampleFreq::Freq24KHz,
        pre_div: 0x02,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq18Mhz,
        rate: SampleFreq::Freq24KHz,
        pre_div: 0x03,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq6144KHz,
        rate: SampleFreq::Freq24KHz,
        pre_div: 0x01,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq3072KHz,
        rate: SampleFreq::Freq24KHz,
        pre_div: 0x01,
        pre_multi: 0x01,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq1536KHz,
        rate: SampleFreq::Freq24KHz,
        pre_div: 0x01,
        pre_multi: 0x02,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq12Mhz,
        rate: SampleFreq::Freq32KHz,
        pre_div: 0x03,
        pre_multi: 0x01,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq18Mhz,
        rate: SampleFreq::Freq32KHz,
        pre_div: 0x03,
        pre_multi: 0x02,
        adc_div: 0x03,
        dac_div: 0x03,
        fs_mode: 0x00,
        lrck_h: 0x02,
        lrck_l: 0xff,
        bclk_div: 0x0c,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq16Mhz,
        rate: SampleFreq::Freq32KHz,
        pre_div: 0x02,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq8192KHz,
        rate: SampleFreq::Freq32KHz,
        pre_div: 0x01,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq6144KHz,
        rate: SampleFreq::Freq32KHz,
        pre_div: 0x03,
        pre_multi: 0x02,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq4096KHz,
        rate: SampleFreq::Freq32KHz,
        pre_div: 0x01,
        pre_multi: 0x01,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq3072KHz,
        rate: SampleFreq::Freq32KHz,
        pre_div: 0x03,
        pre_multi: 0x03,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq2048KHz,
        rate: SampleFreq::Freq32KHz,
        pre_div: 0x01,
        pre_multi: 0x02,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq1536KHz,
        rate: SampleFreq::Freq32KHz,
        pre_div: 0x03,
        pre_multi: 0x03,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x01,
        lrck_h: 0x00,
        lrck_l: 0x7f,
        bclk_div: 0x02,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq1024KHz,
        rate: SampleFreq::Freq32KHz,
        pre_div: 0x01,
        pre_multi: 0x03,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq11Mhz,
        rate: SampleFreq::Freq44KHz,
        pre_div: 0x01,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq5645KHz,
        rate: SampleFreq::Freq44KHz,
        pre_div: 0x01,
        pre_multi: 0x01,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq2822KHz,
        rate: SampleFreq::Freq44KHz,
        pre_div: 0x01,
        pre_multi: 0x02,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq1411KHz,
        rate: SampleFreq::Freq44KHz,
        pre_div: 0x01,
        pre_multi: 0x03,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq12Mhz,
        rate: SampleFreq::Freq48KHz,
        pre_div: 0x01,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq18Mhz,
        rate: SampleFreq::Freq48KHz,
        pre_div: 0x03,
        pre_multi: 0x01,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq6144KHz,
        rate: SampleFreq::Freq48KHz,
        pre_div: 0x01,
        pre_multi: 0x01,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq3072KHz,
        rate: SampleFreq::Freq48KHz,
        pre_div: 0x01,
        pre_multi: 0x02,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq1536KHz,
        rate: SampleFreq::Freq48KHz,
        pre_div: 0x01,
        pre_multi: 0x03,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq12Mhz,
        rate: SampleFreq::Freq64KHz,
        pre_div: 0x03,
        pre_multi: 0x02,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq18Mhz,
        rate: SampleFreq::Freq64KHz,
        pre_div: 0x03,
        pre_multi: 0x02,
        adc_div: 0x03,
        dac_div: 0x03,
        fs_mode: 0x01,
        lrck_h: 0x01,
        lrck_l: 0x7f,
        bclk_div: 0x06,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq16Mhz,
        rate: SampleFreq::Freq64KHz,
        pre_div: 0x01,
        pre_multi: 0x00,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq8192KHz,
        rate: SampleFreq::Freq64KHz,
        pre_div: 0x01,
        pre_multi: 0x01,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq6144KHz,
        rate: SampleFreq::Freq64KHz,
        pre_div: 0x01,
        pre_multi: 0x02,
        adc_div: 0x03,
        dac_div: 0x03,
        fs_mode: 0x01,
        lrck_h: 0x01,
        lrck_l: 0x7f,
        bclk_div: 0x06,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq4096KHz,
        rate: SampleFreq::Freq64KHz,
        pre_div: 0x01,
        pre_multi: 0x02,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq3072KHz,
        rate: SampleFreq::Freq64KHz,
        pre_div: 0x01,
        pre_multi: 0x03,
        adc_div: 0x03,
        dac_div: 0x03,
        fs_mode: 0x01,
        lrck_h: 0x01,
        lrck_l: 0x7f,
        bclk_div: 0x06,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq2048KHz,
        rate: SampleFreq::Freq64KHz,
        pre_div: 0x01,
        pre_multi: 0x03,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq1536KHz,
        rate: SampleFreq::Freq64KHz,
        pre_div: 0x01,
        pre_multi: 0x03,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x01,
        lrck_h: 0x00,
        lrck_l: 0xbf,
        bclk_div: 0x03,
        adc_osr: 0x18,
        dac_osr: 0x18,
    },
    Coefficients {
        mclk: MclkFreq::Freq1024KHz,
        rate: SampleFreq::Freq64KHz,
        pre_div: 0x01,
        pre_multi: 0x03,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x01,
        lrck_h: 0x00,
        lrck_l: 0x7f,
        bclk_div: 0x02,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq11Mhz,
        rate: SampleFreq::Freq88KHz,
        pre_div: 0x01,
        pre_multi: 0x01,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq5645KHz,
        rate: SampleFreq::Freq88KHz,
        pre_div: 0x01,
        pre_multi: 0x02,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq2822KHz,
        rate: SampleFreq::Freq88KHz,
        pre_div: 0x01,
        pre_multi: 0x03,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq1411KHz,
        rate: SampleFreq::Freq88KHz,
        pre_div: 0x01,
        pre_multi: 0x03,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x01,
        lrck_h: 0x00,
        lrck_l: 0x7f,
        bclk_div: 0x02,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq12Mhz,
        rate: SampleFreq::Freq96KHz,
        pre_div: 0x01,
        pre_multi: 0x01,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq18Mhz,
        rate: SampleFreq::Freq96KHz,
        pre_div: 0x03,
        pre_multi: 0x02,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq6144KHz,
        rate: SampleFreq::Freq96KHz,
        pre_div: 0x01,
        pre_multi: 0x02,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq3072KHz,
        rate: SampleFreq::Freq96KHz,
        pre_div: 0x01,
        pre_multi: 0x03,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x00,
        lrck_h: 0x00,
        lrck_l: 0xff,
        bclk_div: 0x04,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
    Coefficients {
        mclk: MclkFreq::Freq1536KHz,
        rate: SampleFreq::Freq96KHz,
        pre_div: 0x01,
        pre_multi: 0x03,
        adc_div: 0x01,
        dac_div: 0x01,
        fs_mode: 0x01,
        lrck_h: 0x00,
        lrck_l: 0x7f,
        bclk_div: 0x02,
        adc_osr: 0x10,
        dac_osr: 0x10,
    },
];
