use proc_bitfield::bitfield;

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ClockManager01(u8): Debug {
        pub mclk_sel: bool @ 7,
        pub mclk_inv: bool @ 6,
        pub mclk_on: bool @ 5,
        pub bclk_on: bool @ 4,
        pub clkadc_on: bool @ 3,
        pub clkdac_on: bool @ 2,
        pub anaclkadc_on: bool @ 1,
        pub anaclkdac_on: bool @ 0,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ClockManager02(u8): Debug {
        pub div_pre: u8 @ 5..=7,
        pub mult_pre: u8 @ 3..=4,
        pub pathsel: bool @ 2,
        pub delysel: u8 @ 0..=1,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ClockManager03(u8): Debug {
        pub adc_fsmode: bool @ 6,
        pub adc_osr: u8 @ 0..=5,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ClockManager04(u8): Debug {
        pub dac_osr: u8 @ 0..=6,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ClockManager05(u8): Debug {
        pub div_clkadc: u8 @ 4..=7,
        pub div_clkdac: u8 @ 0..=3,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ClockManager06(u8): Debug {
        pub bclk_con: bool @ 6,
        pub bclk_inv: bool @ 5,
        pub div_bclk: u8 @ 0..=4,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ClockManager07(u8): Debug {
        pub tri_blrck: bool @ 5,
        pub tri_adcdat: bool @ 4,
        pub div_lrck_8_11: u8 @ 0..=3
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct ClockManager08(u8): Debug {
        pub div_lrck_0_7: u8 @ 0..=7,
    }
}

pub(crate) fn div_lrck(first: ClockManager07, second: ClockManager08) -> u16 {
    u16::from_le_bytes([second.div_lrck_0_7(), first.div_lrck_8_11()])
}

register_group! {
    ClockManager,
    (ClockManager01, 0x01, 0b0000_0000),
    (ClockManager02, 0x02, 0b0000_0000),
    (ClockManager03, 0x03, 0b0001_0000),
    (ClockManager04, 0x04, 0b0001_0000),
    (ClockManager05, 0x05, 0b0000_0000),
    (ClockManager06, 0x06, 0b0000_0011),
    (ClockManager07, 0x07, 0b0000_0000),
    (ClockManager08, 0x08, 0b1111_1111),
}
