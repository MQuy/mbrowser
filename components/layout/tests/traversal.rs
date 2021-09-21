use std::cell::RefCell;
use std::rc::Rc;

use css::error_reporting::{ContextualParseError, ParseErrorReporter};
use css::media_queries::media_list::MediaList;
use css::stylesheets::origin::Origin;
use css::stylesheets::stylesheet::Stylesheet;
use cssparser::SourceLocation;
use dom::global_scope::NodeRef;
use dom::inheritance::Castable;
use dom::parser::DomParser;
use html5ever::driver;
use html5ever::tendril::{StrTendril, TendrilSink};
use layout::rule_colectors::collect_rules;
use layout::style_tree::StyleTree;
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
	let sink = DomParser::new();

	let mut parser = driver::parse_document(sink, Default::default());
	parser.process(StrTendril::from(
		r#"<div style="color: red;">Hello world!</div><p id="hello">my friends</p>"#,
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
	let root = output.document.upcast().first_child().unwrap();
	let style_tree = StyleTree::new(NodeRef(root.clone()), QuirksMode::NoQuirks);
	style_tree.import_user_agent();
	style_tree.add_stylesheet(&stylesheet);

	for node in root.traverse_preorder() {
		if node.node_type_id().is_element() {
			let rules = collect_rules(NodeRef(node), style_tree.stylist());
			println!("{:?}", rules);
		}
	}
}
