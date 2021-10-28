use css::values::Pixel;

#[derive(Debug, Clone)]
pub struct Margin {
	pub margin_top: Pixel,
	pub margin_right: Pixel,
	pub margin_bottom: Pixel,
	pub margin_left: Pixel,
}

impl Default for Margin {
	fn default() -> Self {
		Self {
			margin_top: Default::default(),
			margin_right: Default::default(),
			margin_bottom: Default::default(),
			margin_left: Default::default(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Padding {
	pub padding_top: Pixel,
	pub padding_right: Pixel,
	pub padding_bottom: Pixel,
	pub padding_left: Pixel,
}

impl Default for Padding {
	fn default() -> Self {
		Self {
			padding_top: Default::default(),
			padding_right: Default::default(),
			padding_bottom: Default::default(),
			padding_left: Default::default(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct BoxDimension {
	pub intrinsic_width: Pixel,
	pub width: Pixel,
	pub height: Pixel,
	pub margin: Margin,
	pub padding: Padding,
	pub x: Pixel,
	pub y: Pixel,
	pub constructing_height: Pixel,
	pub constructing_width: Pixel,
}

impl Default for BoxDimension {
	fn default() -> Self {
		Self {
			intrinsic_width: Default::default(),
			width: Default::default(),
			height: Default::default(),
			margin: Default::default(),
			padding: Default::default(),
			x: Default::default(),
			y: Default::default(),
			constructing_height: Default::default(),
			constructing_width: Default::default(),
		}
	}
}

impl BoxDimension {
	pub fn set_width(&mut self, value: Pixel) {
		self.width = value;
	}

	pub fn set_height(&mut self, value: Pixel) {
		self.height = value;
	}

	pub fn set_margin_top(&mut self, value: Pixel) {
		self.margin.margin_top = value;
	}

	pub fn set_margin_right(&mut self, value: Pixel) {
		self.margin.margin_right = value;
	}

	pub fn set_margin_botom(&mut self, value: Pixel) {
		self.margin.margin_bottom = value;
	}

	pub fn set_margin_left(&mut self, value: Pixel) {
		self.margin.margin_left = value;
	}

	pub fn set_padding_top(&mut self, value: Pixel) {
		self.padding.padding_top = value;
	}

	pub fn set_padding_right(&mut self, value: Pixel) {
		self.padding.padding_right = value;
	}

	pub fn set_padding_bottom(&mut self, value: Pixel) {
		self.padding.padding_bottom = value;
	}

	pub fn set_padding_left(&mut self, value: Pixel) {
		self.padding.padding_left = value;
	}

	pub fn set_x(&mut self, value: Pixel) {
		self.x = value;
	}

	pub fn set_y(&mut self, value: Pixel) {
		self.y = value;
	}

	pub fn set_constructing_height(&mut self, value: Pixel) {
		self.constructing_height = value;
	}

	pub fn set_constructing_width(&mut self, value: Pixel) {
		self.constructing_width = value;
	}
}
