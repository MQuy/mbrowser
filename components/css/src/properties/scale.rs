use crate::values::number::Number;

#[derive(Clone)]
#[repr(C, u8)]
pub enum Scale {
    None,
    Scale(Number, Number, Number),
}
