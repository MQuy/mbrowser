use css::values::specified::color::RGBA;
use css::values::{CSSPixel, Pixel};
use euclid::Rect;

pub type LayoutRect = Rect<Pixel, CSSPixel>;

pub struct RectangleDisplayItem {
	pub bounds: LayoutRect,
	pub color: RGBA,
}

pub struct TextDisplayItem {
	pub bounds: LayoutRect,
	pub content: String,
	pub color: RGBA,
}

pub enum DisplayItem {
	Rectangle(RectangleDisplayItem),
	Text(TextDisplayItem),
}
