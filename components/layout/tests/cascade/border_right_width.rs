use std::rc::Rc;

use css::values::computed::line::LineWidth;
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
	assert_eq!(computed_values.get_border_right_width().clone(), LineWidth::Medium);
}

#[test]
#[serial]
fn from_author() {
	let tree = Rc::new(construct_tree(
		r#"<p id="test"></p>"#,
		r#"
#test { border-right-width: thin; }
        "#,
	));
	let dom = find_dom(&tree, "test").unwrap();
	let computed_values = GlobalScope::get_or_init_computed_values(dom.id());
	assert_eq!(computed_values.get_border_right_width().clone(), LineWidth::Thin);
}

#[test]
#[serial]
fn non_inherited() {
	let tree = Rc::new(construct_tree(
		r#"<p id="test1"><span id="test2">Totoland</span></p>"#,
		r#"
#test1 { border-right-width: thin; }
        "#,
	));
	let dom = find_dom(&tree, "test2").unwrap();
	let computed_values = GlobalScope::get_or_init_computed_values(dom.id());
	assert_eq!(computed_values.get_border_right_width().clone(), LineWidth::Medium);
}
