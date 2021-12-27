use std::rc::Rc;

use css::values::computed::length::{LengthPercentage, LengthPercentageOrAuto};
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
	assert_eq!(
		computed_values.get_margin_left().clone(),
		LengthPercentageOrAuto::LengthPercentage(LengthPercentage::AbsoluteLength(0.0))
	);
}

#[test]
#[serial]
fn from_author() {
	let tree = Rc::new(construct_tree(
		r#"<p id="test"></p>"#,
		r#"
#test { margin-left: 100px; }
        "#,
	));
	let dom = find_dom(&tree, "test").unwrap();
	let computed_values = GlobalScope::get_or_init_computed_values(dom.id());
	assert_eq!(
		computed_values.get_margin_left().clone(),
		LengthPercentageOrAuto::LengthPercentage(LengthPercentage::AbsoluteLength(100.0))
	);
}

#[test]
#[serial]
fn non_inherited() {
	let tree = Rc::new(construct_tree(
		r#"<p id="test1"><span id="test2">Totoland</span></p>"#,
		r#"
#test1 { margin-left: 100px; }
        "#,
	));
	let dom = find_dom(&tree, "test2").unwrap();
	let computed_values = GlobalScope::get_or_init_computed_values(dom.id());
	assert_eq!(
		computed_values.get_margin_left().clone(),
		LengthPercentageOrAuto::LengthPercentage(LengthPercentage::AbsoluteLength(0.0))
	);
}
