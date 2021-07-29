use crate::values::number::Number;

use super::font_style::Angle;

#[derive(Clone)]
#[repr(C, u8)]
pub enum Rotate {
    None,
    Rotate(Angle),
    Rotate3D(Number, Number, Number, Angle),
}
