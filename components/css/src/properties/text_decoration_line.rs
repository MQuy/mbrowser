#[derive(Clone)]
#[repr(C)]
pub struct TextDecorationLine {
    bits: u8,
}

/// No text decoration line is specified.
pub const NONE: u8 = 0;
/// underline
pub const UNDERLINE: u8 = 1 << 0;
/// overline
pub const OVERLINE: u8 = 1 << 1;
/// line-through
pub const LINE_THROUGH: u8 = 1 << 2;
/// blink
pub const BLINK: u8 = 1 << 3;
