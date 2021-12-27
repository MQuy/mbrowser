use std::rc::Rc;

use css::values::specified::layout::LineStyle;
use dom::global_scope::GlobalScope;
use serial_test::serial;
use setup::{construct_tree, find_dom};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
#[serial]
fn default() {
	let tree = Rc::new(construct_tree(r#"<p id="test"></p>"#, r#""#));
	let dom = find_dom(&tree, "test").unwrap();
	let computed_values = GlobalScope::get_or_init_computed_values(dom.id());
	assert_eq!(computed_values.get_border_bottom_style().clone(), LineStyle::None);
}

#[test]
#[serial]
fn from_author() {
	let tree = Rc::new(construct_tree(
		r#"<p id="test"></p>"#,
		r#"
#test { border-bottom-style: dotted; }
        "#,
	));
	let dom = find_dom(&tree, "test").unwrap();
	let computed_values = GlobalScope::get_or_init_computed_values(dom.id());
	assert_eq!(computed_values.get_border_bottom_style().clone(), LineStyle::Dotted);
}

#[test]
#[serial]
fn non_inherited() {
	let tree = Rc::new(construct_tree(
		r#"<p id="test1"><span id="test2">Totoland</span></p>"#,
		r#"
#test1 { border-bottom-style: dotted; }
        "#,
	));
	let dom = find_dom(&tree, "test2").unwrap();
	let computed_values = GlobalScope::get_or_init_computed_values(dom.id());
	assert_eq!(computed_values.get_border_bottom_style().clone(), LineStyle::None);
}
