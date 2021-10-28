use std::cell::RefCell;
use std::rc::Rc;

use css::error_reporting::{ContextualParseError, ParseErrorReporter};
use css::media_queries::media_list::MediaList;
use css::stylesheets::origin::Origin;
use css::stylesheets::stylesheet::Stylesheet;
use css::values::Ident;
use cssparser::SourceLocation;
use dom::global_scope::{GlobalScope, NodeRef};
use dom::inheritance::Castable;
use dom::parser::DomParser;
use html5ever::driver;
use html5ever::tendril::{StrTendril, TendrilSink};
use layout::flow::block::BlockLevelBox;
use layout::flow::boxes::BoxClass;
use layout::flow::dimension::BoxDimension;
use layout::flow::inline::InlineLevelBox;
use layout::flow::tree::{BoxTree, PreOrderBoxTreeIterator};
use layout::style_tree::StyleTree;
use selectors::attr::CaseSensitivity;
use selectors::context::QuirksMode;
use selectors::Element;

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

pub fn construct_tree(html: &str, css: &str) -> BoxTree {
	GlobalScope::clear();
	let sink = DomParser::new();

	let mut parser = driver::parse_document(sink, Default::default());
	parser.process(StrTendril::from(html));

	let output = parser.finish();

	let error_reporter = TestingErrorReporter::new();
	let media = Rc::new(MediaList::empty());
	let stylesheet = Stylesheet::from_str(
		css,
		Origin::UserAgent,
		media,
		Some(&error_reporter),
		QuirksMode::NoQuirks,
		0,
	);
	let root = output.document.upcast().first_child().unwrap();
	let style_tree = Rc::new(StyleTree::new(NodeRef(root.clone()), QuirksMode::NoQuirks));
	style_tree.import_user_agent();
	style_tree.add_stylesheet(&stylesheet);
	style_tree.match_rules();
	style_tree.cascade();
	let box_tree = BoxTree::construct(style_tree);
	box_tree.compute_layout();
	box_tree
}

pub fn get_box_dimension(tree: &BoxTree, id: &str) -> Option<BoxDimension> {
	let iter = PreOrderBoxTreeIterator::new(tree.root.clone());
	for child in iter {
		match child.class() {
			BoxClass::Inline => {
				let inline = child
					.as_any()
					.downcast_ref::<InlineLevelBox>()
					.unwrap()
					.dom_node();
				if inline.node_type_id().is_element()
					&& inline.has_id(
						&Ident(id.to_string()),
						CaseSensitivity::AsciiCaseInsensitive,
					) {
					return Some(child.size().clone());
				}
			},
			BoxClass::Block => {
				let block = child
					.as_any()
					.downcast_ref::<BlockLevelBox>()
					.unwrap()
					.dom_node();
				if block.has_id(
					&Ident(id.to_string()),
					CaseSensitivity::AsciiCaseInsensitive,
				) {
					return Some(child.size().clone());
				}
			},
			BoxClass::Anonymous => (),
		}
	}

	None
}
