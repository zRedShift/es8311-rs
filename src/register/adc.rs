use super::{bitfield, Register};

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct Adc01(pub u8): Debug {
        pub adc_ramprate: u8 @ 4..=7,
        pub dmic_sense: bool @ 0,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct Adc02(pub u8): Debug {
        pub adc_sync: bool @ 5,
        pub adc_inv: bool @ 4,
        pub adc_ramclr: bool @ 3,
        pub adc_scale: u8 @ 0..=2,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct Adc03(pub u8): Debug {
        pub adc_volume: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct Adc04(pub u8): Debug {
        pub alc_en: bool @ 7,
        pub adc_automute_en: bool @ 6,
        pub alc_winsize: u8 @ 0..=3,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct Adc05(pub u8): Debug {
        pub alc_maxlevel: u8 @ 4..=7,
        pub alc_minlevel: u8 @ 0..=3,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct Adc06(pub u8): Debug {
        pub adc_automute_ws: u8 @ 4..=7,
        pub adc_automute_ng: u8 @ 0..=3,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct Adc07(pub u8): Debug {
        pub adc_automute_vol: u8 @ 5..=7,
        pub adc_hpfs1: u8 @ 0..=4,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct Adc08(pub u8): Debug {
        pub adc_eqbypass: bool @ 6,
        pub adc_hpf: bool @ 5,
        pub adc_hpfs2: u8 @ 0..=4,
    }
}

register!(Adc01, 0x15, 0b0000_0000);
register!(Adc02, 0x16, 0b0000_0100);
register!(Adc03, 0x17, 0b0000_0000);
register!(Adc04, 0x18, 0b0000_0000);
register!(Adc05, 0x19, 0b0000_0000);
register!(Adc06, 0x1A, 0b0000_0000);
register!(Adc07, 0x1B, 0b0000_1100);
register!(Adc08, 0x1C, 0b0100_1100);
