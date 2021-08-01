use crate::values::CSSFloat;

#[derive(Clone)]
pub enum AngleDimension {
    Deg(CSSFloat),
    Grad(CSSFloat),
    Rad(CSSFloat),
    Turn(CSSFloat),
}

#[derive(Clone)]
pub struct Angle {
    value: AngleDimension,
}

#[derive(Clone)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique(Angle),
}
