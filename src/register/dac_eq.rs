use proc_bitfield::bitfield;

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DacEqualizerB01(u8): Debug {
        pub daceq_b0_24_29: u8 @ 0..=5,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DacEqualizerB02(u8): Debug {
        pub daceq_b0_16_23: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DacEqualizerB03(u8): Debug {
        pub daceq_b0_8_15: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DacEqualizerB04(u8): Debug {
        pub daceq_b0_0_7: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DacEqualizerB11(u8): Debug {
        pub daceq_b1_24_29: u8 @ 0..=5,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DacEqualizerB12(u8): Debug {
        pub daceq_b1_16_23: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DacEqualizerB13(u8): Debug {
        pub daceq_b1_8_15: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DacEqualizerB14(u8): Debug {
        pub daceq_b1_0_7: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DacEqualizerA11(u8): Debug {
        pub daceq_a1_24_29: u8 @ 0..=5,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DacEqualizerA12(u8): Debug {
        pub daceq_a1_16_23: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DacEqualizerA13(u8): Debug {
        pub daceq_a1_8_15: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct DacEqualizerA14(u8): Debug {
        pub daceq_a1_0_7: u8 @ 0..=7,
    }
}

register_group! {
    DacEqualizer,
    (DacEqualizerB01, 0x38, 0b0000_0000),
    (DacEqualizerB02, 0x39, 0b0000_0000),
    (DacEqualizerB03, 0x3A, 0b0000_0000),
    (DacEqualizerB04, 0x3B, 0b0000_0000),
    (DacEqualizerB11, 0x3C, 0b0000_0000),
    (DacEqualizerB12, 0x3D, 0b0000_0000),
    (DacEqualizerB13, 0x3E, 0b0000_0000),
    (DacEqualizerB14, 0x3F, 0b0000_0000),
    (DacEqualizerA11, 0x40, 0b0000_0000),
    (DacEqualizerA12, 0x41, 0b0000_0000),
    (DacEqualizerA13, 0x42, 0b0000_0000),
    (DacEqualizerA14, 0x43, 0b0000_0000),
}
