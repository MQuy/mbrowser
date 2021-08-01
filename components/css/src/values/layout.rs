use super::CSSFloat;

#[derive(Clone)]
#[repr(u8)]
pub enum BorderStyle {
    Hidden,
    None,
    Inset,
    Groove,
    Outset,
    Ridge,
    Dotted,
    Dashed,
    Solid,
    Double,
}

#[derive(Clone)]
#[repr(u8)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
    Auto,
}

#[derive(Clone)]
#[repr(C)]
pub struct Size2D<L> {
    pub width: L,
    pub height: L,
}

/// A specified resolution.
#[derive(Clone, Debug, PartialEq)]
pub enum Resolution {
    /// Dots per inch.
    Dpi(CSSFloat),
    /// An alias unit for dots per pixel.
    X(CSSFloat),
    /// Dots per pixel.
    Dppx(CSSFloat),
    /// Dots per centimeter.
    Dpcm(CSSFloat),
}
