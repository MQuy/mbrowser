use css::values::specified::color::RGBA;
use css::values::{CSSFloat, CSSPixel, Pixel};
use euclid::Rect;

pub type LayoutRect = Rect<Pixel, CSSPixel>;

#[derive(Debug)]
pub struct RectangleDisplayItem {
	pub bounds: LayoutRect,
	pub color: RGBA,
}

#[derive(Debug)]
pub struct TextDisplayItem {
	pub bounds: LayoutRect,
	pub content: String,
	pub color: RGBA,
	pub font_families: Vec<String>,
	pub font_size: CSSFloat,
}

#[derive(Debug)]
pub enum DisplayItem {
	Rectangle(RectangleDisplayItem),
	Text(TextDisplayItem),
}
