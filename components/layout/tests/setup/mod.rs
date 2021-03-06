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
use layout::flow::boxes::{Box, BoxClass};
use layout::flow::fragment::LayoutInfo;
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

pub fn find_box(tree: &BoxTree, id: &str) -> Option<Rc<dyn Box>> {
	let iter = PreOrderBoxTreeIterator::new(tree.root.clone());
	for child in iter {
		match child.class() {
			BoxClass::Inline => {
				let inline = child.as_inline_level_box().dom_node();
				if inline.node_type_id().is_element()
					&& inline.has_id(&Ident(id.to_string()), CaseSensitivity::AsciiCaseInsensitive)
				{
					return Some(child.clone());
				}
			},
			BoxClass::Block => {
				let block = child.as_block_level_box().dom_node();
				if block.has_id(&Ident(id.to_string()), CaseSensitivity::AsciiCaseInsensitive) {
					return Some(child.clone());
				}
			},
			_ => (),
		}
	}

	None
}

pub fn find_dom(tree: &BoxTree, id: &str) -> Option<NodeRef> {
	if let Some(node) = find_box(tree, id) {
		match node.class() {
			BoxClass::Inline => Some(node.as_inline_level_box().dom_node()),
			BoxClass::Block => Some(node.as_block_level_box().dom_node()),
			_ => None,
		}
	} else {
		None
	}
}

pub fn get_layout_info(tree: &BoxTree, id: &str) -> Option<LayoutInfo> {
	if let Some(node) = find_box(tree, id) {
		let value = node.layout_info();
		Some(LayoutInfo {
			width: value.width,
			height: value.height,
			margin: value.margin,
			padding: value.padding,
			..Default::default()
		})
	} else {
		None
	}
}
