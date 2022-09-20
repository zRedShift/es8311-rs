use super::{bitfield, Register};

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerB01(pub u8): Debug {
        pub adceq_b0_24_29: u8 @ 0..=5,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerB02(pub u8): Debug {
        pub adceq_b0_16_23: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerB03(pub u8): Debug {
        pub adceq_b0_8_15: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerB04(pub u8): Debug {
        pub adceq_b0_0_7: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerA11(pub u8): Debug {
        pub adceq_a1_24_29: u8 @ 0..=5,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerA12(pub u8): Debug {
        pub adceq_a1_16_23: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerA13(pub u8): Debug {
        pub adceq_a1_8_15: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerA14(pub u8): Debug {
        pub adceq_a1_0_7: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerA21(pub u8): Debug {
        pub adceq_a2_24_29: u8 @ 0..=5,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerA22(pub u8): Debug {
        pub adceq_a2_16_23: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerA23(pub u8): Debug {
        pub adceq_a2_8_15: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerA24(pub u8): Debug {
        pub adceq_a2_0_7: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerB11(pub u8): Debug {
        pub adceq_b1_24_29: u8 @ 0..=5,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerB12(pub u8): Debug {
        pub adceq_b1_16_23: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerB13(pub u8): Debug {
        pub adceq_b1_8_15: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerB14(pub u8): Debug {
        pub adceq_b1_0_7: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerB21(pub u8): Debug {
        pub adceq_b2_24_29: u8 @ 0..=5,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerB22(pub u8): Debug {
        pub adceq_b2_16_23: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerB23(pub u8): Debug {
        pub adceq_b2_8_15: u8 @ 0..=7,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct AdcEqualizerB24(pub u8): Debug {
        pub adceq_b2_0_7: u8 @ 0..=7,
    }
}

register!(AdcEqualizerB01, 0x1D, 0b0000_0000);
register!(AdcEqualizerB02, 0x1E, 0b0000_0000);
register!(AdcEqualizerB03, 0x1F, 0b0000_0000);
register!(AdcEqualizerB04, 0x20, 0b0000_0000);
register!(AdcEqualizerA11, 0x21, 0b0000_0000);
register!(AdcEqualizerA12, 0x22, 0b0000_0000);
register!(AdcEqualizerA13, 0x23, 0b0000_0000);
register!(AdcEqualizerA14, 0x24, 0b0000_0000);
register!(AdcEqualizerA21, 0x25, 0b0000_0000);
register!(AdcEqualizerA22, 0x26, 0b0000_0000);
register!(AdcEqualizerA23, 0x27, 0b0000_0000);
register!(AdcEqualizerA24, 0x28, 0b0000_0000);
register!(AdcEqualizerB11, 0x29, 0b0000_0000);
register!(AdcEqualizerB12, 0x2A, 0b0000_0000);
register!(AdcEqualizerB13, 0x2B, 0b0000_0000);
register!(AdcEqualizerB14, 0x2C, 0b0000_0000);
register!(AdcEqualizerB21, 0x2D, 0b0000_0000);
register!(AdcEqualizerB22, 0x2E, 0b0000_0000);
register!(AdcEqualizerB23, 0x2F, 0b0000_0000);
register!(AdcEqualizerB24, 0x30, 0b0000_0000);
