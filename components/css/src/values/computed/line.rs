use super::length::NonNegativeLength;

#[derive(Clone, Debug)]
pub enum LineWidth {
	Thin,
	Medium,
	Thick,
	Length(NonNegativeLength),
}
