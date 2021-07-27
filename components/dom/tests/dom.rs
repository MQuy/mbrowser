use dom::{inheritance::upcast, node::Node, parser::DomParser};
use html5ever::{
    driver,
    tendril::{StrTendril, TendrilSink},
};

#[test]
fn check_dom() {
    let sink = DomParser::default();

    let mut parser = driver::parse_document(sink, Default::default());
    parser.process(StrTendril::from("<div>Hello world!</div>"));

    let output = parser.finish();
    walk(upcast(output.document).as_ref());
}

fn walk(node: &Node) {
    println!("id {:?}", node.get_node_type_id());
    for ele in node.children() {
        walk(ele.as_ref());
    }
}
