use std::cell::RefCell;
use std::rc::{Rc, Weak};

use css::error_reporting::{ContextualParseError, ParseErrorReporter};
use cssparser::SourceLocation;

use crate::document::Document;

#[derive(Debug)]
pub struct Window {
	error_reporter: CSSErrorReporter,
	document: Weak<Document>,
}

impl Window {
	pub fn new(document: Rc<Document>, error_reporter: CSSErrorReporter) -> Self {
		Window {
			error_reporter,
			document: Rc::downgrade(&document),
		}
	}

	pub fn error_reporter(&self) -> &CSSErrorReporter {
		&self.error_reporter
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
