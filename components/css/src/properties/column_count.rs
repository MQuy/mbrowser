use crate::values::number::PositiveInteger;

#[derive(Clone)]
pub enum ColumnCount {
    Integer(PositiveInteger),
    Auto,
}
