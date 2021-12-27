use super::length::NonNegativeLength;

#[derive(Clone, Debug, PartialEq)]
pub enum LineWidth {
	Thin,
	Medium,
	Thick,
	Length(NonNegativeLength),
}
