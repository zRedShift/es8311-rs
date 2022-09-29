use proc_bitfield::bitfield;

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Dac01(u8): Debug {
        pub dac_dsmmute_to: bool @ 7,
        pub dac_dsmmute: bool @ 6,
        pub dac_demmute: bool @ 5,
        pub dac_inv: bool @ 4,
        pub dac_ramclr: bool @ 3,
        pub dac_dsmdith_off: bool @ 2,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Dac02(u8): Debug {
        pub dac_volume: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Dac03(u8): Debug {
        pub dac_offset: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Dac04(u8): Debug {
        pub drc_en: bool @ 7,
        pub drc_winsize: u8 @ 0..=3,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Dac05(u8): Debug {
        pub drc_maxlevel: u8 @ 4..=7,
        pub drc_minlevel: u8 @ 0..=3,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Dac06(u8): Debug {
        pub dac_reserved: u8 [ro] @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Dac07(u8): Debug {
        pub dac_ramprate: u8 @ 4..=7,
        pub dac_eqbypass: bool @ 3,
    }
}

register_group! {
    Dac,
    (Dac01, 0x31, 0b0000_0000),
    (Dac02, 0x32, 0b0000_0000),
    (Dac03, 0x33, 0b0000_0000),
    (Dac04, 0x34, 0b0000_0000),
    (Dac05, 0x35, 0b0000_0000),
    (Dac06, 0x36, 0b0000_0000),
    (Dac07, 0x37, 0b0000_1000),
}
