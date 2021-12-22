use std::rc::Rc;

use css::properties::longhands::display::{Display, DisplayBasic, DisplayInside, DisplayLegacy, DisplayOutside};
use dom::global_scope::GlobalScope;
use serial_test::serial;
use setup::{construct_tree, find_dom};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
#[serial]
fn default() {
	let tree = Rc::new(construct_tree(r#"<span id="test"></span>"#, r#""#));
	let dom = find_dom(&tree, "test").unwrap();
	let computed_values = GlobalScope::get_or_init_computed_values(dom.id());
	assert_eq!(
		computed_values.get_display().clone(),
		Display::Basic(DisplayBasic {
			inside: Some(DisplayInside::Flow),
			outside: Some(DisplayOutside::Inline),
		})
	);
}

#[test]
#[serial]
fn from_author() {
	let tree = Rc::new(construct_tree(
		r#"<span id="test"></span>"#,
		r#"
#test { display: inline-block; }
        "#,
	));
	let dom = find_dom(&tree, "test").unwrap();
	let computed_values = GlobalScope::get_or_init_computed_values(dom.id());
	assert_eq!(
		computed_values.get_display().clone(),
		Display::Legacy(DisplayLegacy::InlineBlock)
	);
}

#[test]
#[serial]
fn inherited() {
	let tree = Rc::new(construct_tree(
		r#"<span id="test1"><span id="test2">Totoland</span></span>"#,
		r#"
#test1 { display: block; }
        "#,
	));
	let dom = find_dom(&tree, "test2").unwrap();
	let computed_values = GlobalScope::get_or_init_computed_values(dom.id());
	assert_eq!(
		computed_values.get_display().clone(),
		Display::Basic(DisplayBasic {
			inside: Some(DisplayInside::Flow),
			outside: Some(DisplayOutside::Inline),
		})
	);
}
