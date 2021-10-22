use std::cell::RefCell;
use std::rc::Rc;

use css::error_reporting::{ContextualParseError, ParseErrorReporter};
use css::media_queries::media_list::MediaList;
use css::stylesheets::origin::Origin;
use css::stylesheets::stylesheet::Stylesheet;
use cssparser::SourceLocation;
use dom::global_scope::{GlobalScope, NodeRef};
use dom::inheritance::Castable;
use dom::node::Node;
use dom::parser::DomParser;
use html5ever::driver;
use html5ever::tendril::{StrTendril, TendrilSink};
use layout::flow_tree::BoxTree;
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

fn log(node: Rc<Node>, depth: usize) {
	let indent: String = std::iter::repeat("  ").take(depth).collect();
	let used_values = GlobalScope::get_or_init_used_values(node.id());
	println!(
		"{}{:?} {:?}",
		indent,
		node.node_type_id(),
		used_values.get_width()
	);
	for child in node.children() {
		log(child, depth + 1);
	}
}

#[test]
fn block_box_contains_inline_block_box() {
	let sink = DomParser::new();

	let mut parser = driver::parse_document(sink, Default::default());
	parser.process(StrTendril::from(
		r#"
<div style="color: red;">
    Hello world!
    <div id="hello" style="display: inline-block">
        <div>Echo from the past</div>
    </div>
    <p><span>Totoland</span></p>
</div>"#,
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
	log(root, 0);
}
