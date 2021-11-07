use std::rc::Rc;

use dom::global_scope::GlobalScope;
use serial_test::serial;
use setup::{construct_tree, find_dom};

#[path = "./setup/mod.rs"]
mod setup;

#[test]
#[serial]
fn font_families() {
	let tree = Rc::new(construct_tree(
		r#"
<div style="color: red;">
    Hello world!
    <div style="display: inline-block; height: 40px">
        <div>Echo from the past</div>
    </div>
    <p id="test"><span>Totoland</span></p>
</div>"#,
		r#""#,
	));
	let dom = find_dom(&tree, "test").unwrap();
	let computed_values = GlobalScope::get_or_init_computed_values(dom.id());
	assert_eq!(computed_values.get_font_families(), &vec!["system-ui"]);
}
