use std::cell::RefCell;
use std::rc::Rc;

use css::error_reporting::{ContextualParseError, ParseErrorReporter};
use css::media_queries::media_list::MediaList;
use css::stylesheets::origin::Origin;
use css::stylesheets::stylesheet::Stylesheet;
use css::stylist::Stylist;
use cssparser::SourceLocation;
use dom::global_scope::NodeRef;
use dom::inheritance::Castable;
use dom::parser::DomParser;
use html5ever::driver;
use html5ever::tendril::{StrTendril, TendrilSink};
use layout::traversal::compute_values;
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

#[test]
fn demo() {
	let sink = DomParser::default();

	let mut parser = driver::parse_document(sink, Default::default());
	parser.process(StrTendril::from(
		r#"<div style="color: red;">Hello world!</div>"#,
	));

	let output = parser.finish();

	let error_reporter = TestingErrorReporter::new();
	let media = Rc::new(MediaList::empty());
	let stylesheet = Stylesheet::from_str(
		r#"
#hello {
	align-content: normal;
}
    "#,
		Origin::UserAgent,
		media,
		Some(&error_reporter),
		QuirksMode::NoQuirks,
		5,
	);
	let mut stylist = Stylist::new(QuirksMode::NoQuirks);
	stylist.add_stylesheet(Rc::new(stylesheet));

	let root = output.document.upcast().get_first_child().unwrap();
	for node in root.traverse_preorder() {
		if node.get_node_type_id().is_element() {
			compute_values(NodeRef(node), &stylist)
		}
	}
}
