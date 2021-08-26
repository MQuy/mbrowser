use dom::element::Element;
use dom::inheritance::{upcast, Castable};
use dom::node::Node;
use dom::nodetype::NodeTypeId;
use dom::parser::DomParser;
use html5ever::driver;
use html5ever::tendril::{StrTendril, TendrilSink};

#[test]
fn check_parser() {
	let sink = DomParser::default();

	let mut parser = driver::parse_document(sink, Default::default());
	parser.process(StrTendril::from(r#"<div class="hello">Hello world!</div>"#));

	let output = parser.finish();
	walk(upcast(output.document).as_ref(), 0);
}

fn walk(node: &Node, depth: usize) {
	let indent: String = std::iter::repeat("  ").take(depth).collect();
	let attrs: Vec<String> = match node.get_node_type_id() {
		NodeTypeId::Element(_) => node
			.downcast::<Element>()
			.get_attrs()
			.iter()
			.map(|attr| format!("{}={}", attr.get_name(), attr.get_value()))
			.collect(),
		_ => vec![],
	};
	println!("{} {:?} {:?}", indent, node.get_node_type_id(), attrs);

	for ele in node.children() {
		walk(ele.as_ref(), depth + 1);
	}
}
