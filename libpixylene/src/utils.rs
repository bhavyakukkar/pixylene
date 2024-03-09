pub mod messages {
    pub const U32TOUSIZE: &str =
        "Cannot parse u32 to usize: pixylene requires at least a 32-bit system";
    pub const U16TOISIZE: &str =
        "Cannot parse u16 to isize: pixylene requires at least a 32-bit system";
    pub const DIVZERO: &str =
        "Clearly dividing by 255 not 0";
    pub const CMPSTMSG: &str =
        "Since (frac_a + frac_b) is in range (0,255), range of computed composite is guaranteed \
        to be in range (0,255)";
    pub const SUM255: &str =
        "Guaranteed to sum to 255";
    pub const PCOORD_NOTFAIL: &str =
        "This shouldn't fail since PCoord was constructed from non-zero literals";
    pub const EQUIPPEDISINPALETTE: &str =
        "Equipped index will always have a value in the palette color map";
    pub const PALETTELEN: &str =
        "The Palette length has been used to verify that this operation shouldn't fail";
    //pub const USIZETOISIZE: &str =
    //    "Cannot parse u16 to isize for some reason";
}
