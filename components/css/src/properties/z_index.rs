use crate::values::number::Integer;

#[derive(Clone)]
#[repr(C, u8)]
pub enum ZIndex {
    Integer(Integer),
    Auto,
}
