#[derive(Clone)]
#[repr(C)]
pub enum TextTransformCase {
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}

#[derive(Clone)]
#[repr(C)]
pub struct TextTransformSize {
    bits: u8,
}

pub const FULL_WIDTH: u8 = 1 << 0;
pub const FULL_SIZE_KANA: u8 = 1 << 1;

#[derive(Clone)]
#[repr(C)]
pub struct TextTransformValue {
    case: TextTransformCase,
    size: TextTransformSize,
}

#[derive(Clone)]
#[repr(C)]
pub enum TextTransform {
    None,
    Transform(TextTransformValue),
}
