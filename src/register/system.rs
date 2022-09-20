use super::{bitfield, Register};

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct System01(pub u8): Debug {
        pub pwrup_a: u8 @ 3..=7,
        pub pwrup_b_1_3: u8 @ 0..=2,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct System02(pub u8): Debug {
        pub pwrup_b_0: u8 @ 7; 1,
        pub pwrup_c: u8 @ 0..=6,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct System03(pub u8): Debug {
        pub pdn_ana: bool @ 7,
        pub pdn_ibiasgen: bool @ 6,
        pub pdn_adcbiasgen: bool @ 5,
        pub pdn_adcverfgen: bool @ 4,
        pub pdn_dacvrefgen: bool @ 3,
        pub pdn_vref: bool @ 2,
        pub vmidsel: u8 @ 0..=1,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct System04(pub u8): Debug {
        pub pdn_pga: bool @ 6,
        pub pdn_mod: bool @ 5,
        pub rst_mod: bool @ 4,
        pub vroi: bool @ 3,
        pub lpvrefbuf: bool @ 2,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct System05(pub u8): Debug {
        pub lpdac: bool @ 7,
        pub lppga: bool @ 6,
        pub lppgaout: bool @ 5,
        pub lpvcmmod: bool @ 4,
        pub lpadcvrp: bool @ 3,
        pub lpdacvrp: bool @ 2,
        pub lpflash: bool @ 1,
        pub lpint1: bool @ 0,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct System06(pub u8): Debug {
        pub syncmode: bool @ 7,
        pub vmidlow: u8 @ 5..=6,
        pub dac_ibias_sw: bool @ 4,
        pub ibias_sw: u8 @ 2..=3,
        pub vx2off: bool @ 1,
        pub vx1sel: bool @ 0,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct System07(pub u8): Debug {
        pub vsel: u8 [ro] @ 0..=6,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct System08(pub u8): Debug {
        pub pdn_dac: bool @ 1,
        pub enrefr: bool @ 0,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct System09(pub u8): Debug {
        pub hpsw: bool @ 4,
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub(crate) struct System10(pub u8): Debug {
        pub dmic_on: bool @ 6,
        pub linsel: bool @ 4,
        pub pgagain: u8 @ 0..=3,
    }
}

register!(System01, 0x0B, 0b0000_0000);
register!(System02, 0x0C, 0b0010_0000);
register!(System03, 0x0D, 0b1111_1100);
register!(System04, 0x0E, 0b0110_1010);
register!(System05, 0x0F, 0b0000_0000);
register!(System06, 0x10, 0b0001_0011);
register!(System07, 0x11, 0b0111_1100);
register!(System08, 0x12, 0b0000_0010);
register!(System09, 0x13, 0b0100_0000);
register!(System10, 0x14, 0b0001_0000);

pub(crate) struct System(System01, System02, System03, System04);

// impl core::fmt::Debug for System {
//     fn fmt(&self, f: &mut ::core::fmt::Formatter) -> core::fmt::Result {
//         impl ::core::fmt::Debug for System10 {
//             fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//                 let mut debug = f.debug_tuple(stringify!(System));
//                 f.debug_struct(stringify!(System10))
//                     .field("0", &self.0)
//                     .field(stringify!(dmic_on), &self.dmic_on())
//                     .field(stringify!(linsel), &self.linsel())
//                     .field(stringify!(pgagain), &self.pgagain())
//                     .finish()
//             }
//         }
//         todo!()
//     }
// }
