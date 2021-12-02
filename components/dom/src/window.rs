use std::cell::RefCell;
use std::rc::{Rc, Weak};

use css::error_reporting::{ContextualParseError, ParseErrorReporter};
use css::values::CSSPixel;
use cssparser::SourceLocation;
use euclid::{Length, Point2D, Rect, Scale, Size2D};

use crate::document::Document;

type Pixel = Length<f32, CSSPixel>;

pub const DEFAULT_WIDTH: f32 = 1200.0;
pub const DEFAULT_HEIGHT: f32 = 800.0;
pub const DEFAULT_RATIO: f32 = 1.0;

#[derive(Debug)]
pub struct Window {
	error_reporter: CSSErrorReporter,
	document: Weak<Document>,
	window_size: WindowSize,
	viewport: Rect<f32, Pixel>,
}

impl Window {
	pub fn new(document: Rc<Document>, error_reporter: CSSErrorReporter) -> Self {
		Window {
			error_reporter,
			document: Rc::downgrade(&document),
			window_size: WindowSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT, DEFAULT_RATIO),
			viewport: Rect::new(Point2D::new(0.0, 0.0), Size2D::new(DEFAULT_WIDTH, DEFAULT_HEIGHT)),
		}
	}

	pub fn error_reporter(&self) -> &CSSErrorReporter {
		&self.error_reporter
	}

	pub fn viewport(&self) -> &Rect<f32, Pixel> {
		&self.viewport
	}
}

/// https://www.w3.org/TR/css-device-adapt/#the-viewport
#[derive(Debug)]
pub struct WindowSize {
	pub initial_viewport: Size2D<f32, CSSPixel>,
	pub device_pixel_ratio: Scale<f32, CSSPixel, Pixel>,
}

impl WindowSize {
	pub fn new(width: f32, height: f32, ratio: f32) -> Self {
		Self {
			initial_viewport: Size2D::new(width, height),
			device_pixel_ratio: Scale::new(ratio),
		}
	}
}

#[derive(Debug)]
pub struct CSSError {
	pub line: u32,
	pub column: u32,
	pub message: String,
}

#[derive(Debug)]
pub struct CSSErrorReporter {
	errors: RefCell<Vec<CSSError>>,
}

impl CSSErrorReporter {
	pub fn new() -> Self {
		CSSErrorReporter {
			errors: RefCell::new(Vec::new()),
		}
	}
}

impl ParseErrorReporter for CSSErrorReporter {
	fn report_error(&self, location: SourceLocation, error: ContextualParseError) {
		self.errors.borrow_mut().push(CSSError {
			line: location.line,
			column: location.column,
			message: error.to_string(),
		})
	}
}
