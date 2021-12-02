use std::cell::RefCell;
use std::rc::Rc;

use css::error_reporting::{ContextualParseError, ParseErrorReporter};
use css::media_queries::media_list::MediaList;
use css::parser::ParseError;
use css::stylesheets::origin::Origin;
use css::stylesheets::stylesheet::{ParserContext, Stylesheet};
use cssparser::{Parser, ParserInput, SourceLocation, ToCss};
use dyn_fmt::AsStrFormatExt;
use selectors::context::QuirksMode;

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

#[allow(dead_code)]
pub fn parse_value<'i, F, T: ToCss>(text: &'i str, func: F) -> Result<String, ParseError<'i>>
where
	F: for<'t> Fn(&ParserContext, &mut Parser<'i, 't>) -> Result<T, ParseError<'i>>,
{
	let error_reporter = TestingErrorReporter::new();
	let mut input = ParserInput::new_with_line_number_offset(text, 0);
	let mut input = Parser::new(&mut input);

	let context = ParserContext::new(Origin::UserAgent, None, QuirksMode::NoQuirks, Some(&error_reporter));
	let ret = func(&context, &mut input)?;
	Ok(ret.to_css_string())
}

fn trim(text: &str) -> String {
	fn is_newline_or_space(ch: char) -> bool {
		ch == '\n' || ch == ' ' || ch == '\t'
	}

	text.trim_start_matches(is_newline_or_space)
		.trim_end_matches(is_newline_or_space)
		.to_owned()
}

pub fn assert_css<T: ToString>(style: &T, text: &str) {
	assert_eq!(trim(&style.to_string()), trim(text))
}

#[allow(dead_code)]
pub fn assert_property(template: &str, input: &str, output: &str) {
	let cin = template.format(&[input]);
	let cout = template.format(&[output]);
	let (stylesheet, _) = parse(&cin);
	assert_css(&stylesheet, &cout);
}

#[allow(unused_macros)]
macro_rules! test_property {
	($name: ident, $data: ident) => {
		#[test]
		pub fn $name() {
			for (input, output) in $data().iter() {
				assert_property(TEMPLATE, input, output);
			}
		}
	};
}
