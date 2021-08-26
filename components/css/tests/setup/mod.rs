use std::cell::RefCell;
use std::rc::Rc;

use css::error_reporting::{ContextualParseError, ParseErrorReporter};
use css::media_queries::media_list::MediaList;
use css::stylesheets::origin::Origin;
use css::stylesheets::stylesheet::{QuirksMode, Stylesheet};
use cssparser::SourceLocation;

#[derive(Debug)]
pub struct CSSError {
	pub line: u32,
	pub column: u32,
	pub message: String,
}

pub struct TestingErrorReporter {
	errors: RefCell<Vec<CSSError>>,
}

impl TestingErrorReporter {
	pub fn new() -> Self {
		TestingErrorReporter {
			errors: RefCell::new(Vec::new()),
		}
	}
}

impl ParseErrorReporter for TestingErrorReporter {
	fn report_error(&self, location: SourceLocation, error: ContextualParseError) {
		self.errors.borrow_mut().push(CSSError {
			line: location.line,
			column: location.column,
			message: error.to_string(),
		})
	}
}

pub fn parse(css: &str) -> (Stylesheet, TestingErrorReporter) {
	let error_reporter = TestingErrorReporter::new();
	let media = Rc::new(MediaList::empty());
	(
		Stylesheet::from_str(
			css,
			Origin::UserAgent,
			media,
			Some(&error_reporter),
			QuirksMode::NoQuirks,
			5,
		),
		error_reporter,
	)
}

fn trim(text: &str) -> String {
	fn is_newline_or_space(ch: char) -> bool {
		ch == '\n' || ch == ' ' || ch == '\t'
	}

	text.trim_start_matches(is_newline_or_space)
		.trim_end_matches(is_newline_or_space)
		.to_owned()
}

pub fn assert_stylesheet(style: &Stylesheet, text: &str) {
	assert_eq!(trim(&style.to_string()), trim(text))
}
