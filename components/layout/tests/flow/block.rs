use std::rc::Rc;

use css::properties::longhands::font_size::DEFAULT_FONT_SIZE;
use css::values::{Pixel, PIXEL_ZERO};
use dom::window;
use layout::flow::boxes::Box;
use layout::flow::fragment::Fragment;
use layout::text::TextUI;
use serial_test::serial;

use self::setup::{construct_tree, find_box};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
#[serial]
fn block_level_with_auto_width() {
	let tree = construct_tree(r#"<div id="test">hello world</div>"#, r#""#);
	let node = find_box(&tree, "test").unwrap();
	let fragment = node.as_block_level_box().fragment();
	let (_, height) = TextUI::new().measure_size("hello world", &vec!["system-ui"], DEFAULT_FONT_SIZE);
	assert_eq!(fragment.total_width(), Pixel::new(window::DEFAULT_WIDTH));
	assert_eq!(fragment.total_height(), Pixel::new(height));
	assert_eq!(fragment.x(), PIXEL_ZERO);
	assert_eq!(fragment.y(), PIXEL_ZERO);
}

#[test]
#[serial]
fn block_level_with_auto_width_and_include_margin_padding() {
	let tree = construct_tree(
		r#"<div id="test">hello world</div>"#,
		r#"#test { padding: 100px; margin: 50px }"#,
	);
	let node = find_box(&tree, "test").unwrap();
	let fragment = node.as_block_level_box().fragment();
	let (_, height) = TextUI::new().measure_size("hello world", &vec!["system-ui"], DEFAULT_FONT_SIZE);
	assert_eq!(fragment.total_width(), Pixel::new(window::DEFAULT_WIDTH));
	assert_eq!(fragment.total_height(), Pixel::new(height + 300.0));
	assert_eq!(fragment.width(), Pixel::new(window::DEFAULT_WIDTH - 300.0));
	assert_eq!(fragment.height(), Pixel::new(height));
	assert_eq!(fragment.x(), PIXEL_ZERO);
	assert_eq!(fragment.y(), PIXEL_ZERO);
}

#[test]
#[serial]
fn block_level_with_fixed_width() {
	let tree = construct_tree(r#"<div id="test">hello world</div>"#, r#"#test { width: 400px; }"#);
	let node = find_box(&tree, "test").unwrap();
	let fragment = node.as_block_level_box().fragment();
	assert_eq!(fragment.width(), Pixel::new(400.0));
}

#[test]
#[serial]
fn block_level_with_percentage_width() {
	let tree = construct_tree(r#"<div id="test">hello world</div>"#, r#"#test { width: 60%; }"#);
	let node = find_box(&tree, "test").unwrap();
	let fragment = node.as_block_level_box().fragment();
	assert_eq!(fragment.width(), Pixel::new(window::DEFAULT_WIDTH * 0.6));
}

#[test]
#[serial]
fn block_level_with_auto_width_left_right_auto_to_zero() {
	let tree = construct_tree(
		r#"<div id="test"></div>"#,
		r#"
body { width: 400px; }
#test { width: auto; margin: 0 auto; }
        "#,
	);
	let node = find_box(&tree, "test").unwrap();
	let fragment = node.as_block_level_box().fragment();
	assert_eq!(fragment.margin.left, PIXEL_ZERO);
	assert_eq!(fragment.margin.right, PIXEL_ZERO);
}

#[test]
#[serial]
fn block_level_with_non_auto_width_and_over_constrained_left_right_auto_to_zero() {
	let tree = construct_tree(
		r#"<div id="test"></div>"#,
		r#"
body { width: 400px; }
#test { width: 360px; padding: 0 20px; margin: 0 auto; }
        "#,
	);
	let node = find_box(&tree, "test").unwrap();
	let fragment = node.as_block_level_box().fragment();
	assert_eq!(fragment.margin.left, PIXEL_ZERO);
	assert_eq!(fragment.margin.right, PIXEL_ZERO);
}

#[test]
#[serial]
fn block_level_with_non_auto_width_and_left_auto_to_leftover() {
	let tree = construct_tree(
		r#"<div id="test"></div>"#,
		r#"
body { width: 400px; }
#test { width: 360px; padding: 0 10px; margin-left: auto; }
        "#,
	);
	let node = find_box(&tree, "test").unwrap();
	let fragment = node.as_block_level_box().fragment();
	assert_eq!(fragment.margin.left, Pixel::new(20.0));
	assert_eq!(fragment.margin.right, PIXEL_ZERO);
}

#[test]
#[serial]
fn block_level_with_non_auto_width_and_left_right_auto_to_divided_equal() {
	let tree = construct_tree(
		r#"<div id="test"></div>"#,
		r#"
body { width: 400px; }
#test { width: 360px; padding: 0 10px; margin: 0 auto; }
        "#,
	);
	let node = find_box(&tree, "test").unwrap();
	let fragment = node.as_block_level_box().fragment();
	assert_eq!(fragment.margin.left, Pixel::new(10.0));
	assert_eq!(fragment.margin.right, Pixel::new(10.0));
}

#[test]
#[serial]
fn block_level_with_auto_height() {
	let tree = construct_tree(
		r#"
<div id="test">
    <div style="height: 40px"></div>
    <div style="height: 60px"></div>
</div>
        "#,
		r#""#,
	);
	let node = find_box(&tree, "test").unwrap();
	let fragment = node.as_block_level_box().fragment();
	assert_eq!(fragment.height(), Pixel::new(100.0));
}

#[test]
#[serial]
fn block_level_with_auto_height_with_margin_padding_children() {
	let tree = construct_tree(
		r#"
<div id="test">
    <div style="height: 40px; margin: 10px 0"></div>
    <div style="height: 60px; padding: 15px 0"></div>
</div>
        "#,
		r#""#,
	);
	let node = find_box(&tree, "test").unwrap();
	let fragment = node.as_block_level_box().fragment();
	assert_eq!(fragment.height(), Pixel::new(150.0));
}

#[test]
#[serial]
fn block_level_with_fixed_height() {
	let tree = construct_tree(r#"<div id="test"></div>"#, r#"#test { height: 400px; }"#);
	let node = find_box(&tree, "test").unwrap();
	let fragment = node.as_block_level_box().fragment();
	assert_eq!(fragment.height(), Pixel::new(400.0));
}

#[test]
#[serial]
fn block_level_with_percentage_height() {
	let tree = construct_tree(
		r#"<div id="test"></div>"#,
		r#"
html, body { height: 100%; }
#test { height: 40%; }
        "#,
	);
	let node = find_box(&tree, "test").unwrap();
	let fragment = node.as_block_level_box().fragment();
	assert_eq!(fragment.height(), Pixel::new(window::DEFAULT_HEIGHT * 0.4));
}

#[test]
#[serial]
fn block_level_with_top_bottom_auto_to_zero() {
	let tree = construct_tree(r#"<div id="test"></div>"#, r#"#test { height: 400px; margin: auto 0 }"#);
	let node = find_box(&tree, "test").unwrap();
	let fragment = node.as_block_level_box().fragment();
	assert_eq!(fragment.height(), Pixel::new(400.0));
	assert_eq!(fragment.margin.top, PIXEL_ZERO);
	assert_eq!(fragment.margin.bottom, PIXEL_ZERO);
}

#[test]
#[serial]
fn block_level_with_fixed_margin_padding() {
	let tree = construct_tree(
		r#"<div id="test"></div>"#,
		r#"
#test { width: auto; margin: 5px 10px 15px 20px; padding: 30px 35px 40px 45px }
        "#,
	);
	let node = find_box(&tree, "test").unwrap();
	let fragment = node.as_block_level_box().fragment();
	assert_eq!(fragment.margin.top, Pixel::new(5.0));
	assert_eq!(fragment.margin.right, Pixel::new(10.0));
	assert_eq!(fragment.margin.bottom, Pixel::new(15.0));
	assert_eq!(fragment.margin.left, Pixel::new(20.0));
	assert_eq!(fragment.padding.top, Pixel::new(30.0));
	assert_eq!(fragment.padding.right, Pixel::new(35.0));
	assert_eq!(fragment.padding.bottom, Pixel::new(40.0));
	assert_eq!(fragment.padding.left, Pixel::new(45.0));
}

#[test]
#[serial]
fn block_level_with_percentage_margin_padding() {
	let tree = construct_tree(
		r#"<div id="test"></div>"#,
		r#"
#test { width: auto; margin: 1% 2% 3% 4%; padding: 11% 12% 13% 14% }
        "#,
	);
	let node = find_box(&tree, "test").unwrap();
	let fragment = node.as_block_level_box().fragment();
	assert_eq!(fragment.margin.top, Pixel::new(window::DEFAULT_WIDTH * 0.01));
	assert_eq!(fragment.margin.right, Pixel::new(window::DEFAULT_WIDTH * 0.02));
	assert_eq!(fragment.margin.bottom, Pixel::new(window::DEFAULT_WIDTH * 0.03));
	assert_eq!(fragment.margin.left, Pixel::new(window::DEFAULT_WIDTH * 0.04));
	assert_eq!(fragment.padding.top, Pixel::new(window::DEFAULT_WIDTH * 0.11));
	assert_eq!(fragment.padding.right, Pixel::new(window::DEFAULT_WIDTH * 0.12));
	assert_eq!(fragment.padding.bottom, Pixel::new(window::DEFAULT_WIDTH * 0.13));
	assert_eq!(fragment.padding.left, Pixel::new(window::DEFAULT_WIDTH * 0.14));
}

#[test]
#[serial]
fn block_level_as_first_child_top_left_position() {
	let tree = construct_tree(r#"<div id="test">hello world</div>"#, r#""#);
	let node = find_box(&tree, "test").unwrap();
	let fragment = node.as_block_level_box().fragment();
	assert_eq!(fragment.x(), Pixel::new(0.0));
	assert_eq!(fragment.y(), Pixel::new(0.0));
}

#[test]
#[serial]
fn block_level_as_second_child_position() {
	let tree = construct_tree(
		r#"
<div id="test1">hello world</div>
<div id="test2">hello world</div>
        "#,
		r#"
#test1 { height: 40px; padding: 10px; margin: 15px }
        "#,
	);
	let node = find_box(&tree, "test2").unwrap();
	let fragment = node.as_block_level_box().fragment();
	assert_eq!(fragment.x(), Pixel::new(0.0));
	assert_eq!(fragment.y(), Pixel::new(90.0));
}

#[test]
#[serial]
fn block_box_contains_inline_block_box() {
	let tree = Rc::new(construct_tree(
		r#"
<div style="color: red;">
    Hello world!
    <div id="hello" style="display: inline-block; height: 40px">
    </div>
    <p id="test"><span>Totoland</span></p>
</div>"#,
		r#""#,
	));
	let node = find_box(&tree, "test").unwrap();
	let fragment = node.as_block_level_box().fragment();
	assert_eq!(fragment.x(), Pixel::new(0.0));
	assert_eq!(fragment.y(), Pixel::new(40.0));
}

#[test]
#[serial]
fn block_box_contains_two_spans_same_line() {
	let tree = Rc::new(construct_tree(
		r#"
<div id="test" style="width: 200px;">
    <span>hello darkness</span> <span>my old friend</span>
</div>"#,
		r#""#,
	));
	let (width, _) = TextUI::new().measure_size("hello darkness", &vec!["system-ui"], DEFAULT_FONT_SIZE);
	let node = find_box(&tree, "test").unwrap();
	let node = node.as_block_level_box();
	let lines = node.lines();
	assert_eq!(lines.len(), 1);
	let fragments = lines[0].fragments();
	assert_eq!(fragments.len(), 2);
	assert_eq!(fragments[1].borrow().x(), Pixel::new(width));
}

#[test]
#[serial]
fn block_box_contains_textrun_span_multilines() {
	let tree = Rc::new(construct_tree(
		r#"
<div id="test" style="width: 150px;">
    <span>hello darkness my old friend</span>
</div>"#,
		r#""#,
	));
	let (width, height) = TextUI::new().measure_size("hello darkness my old ", &vec!["system-ui"], DEFAULT_FONT_SIZE);
	let node = find_box(&tree, "test").unwrap();
	let node = node.as_block_level_box();
	let lines = node.lines();
	assert_eq!(lines.len(), 2);
	assert_eq!(lines[0].width(), Pixel::new(width));
	assert_eq!(lines[1].y(), Pixel::new(height));
}

#[test]
#[serial]
fn block_box_contains_textrun_span_multilines_with_margin() {
	let tree = Rc::new(construct_tree(
		r#"
<div id="test" style="width: 150px;">
    <span style="margin: 0 10px">hello darkness my old friend</span>
    <span>again</span>
</div>"#,
		r#""#,
	));
	let (width1, height1) = TextUI::new().measure_size("hello darkness my old ", &vec!["system-ui"], DEFAULT_FONT_SIZE);
	let (width2, _) = TextUI::new().measure_size("friendagain", &vec!["system-ui"], DEFAULT_FONT_SIZE);
	let node = find_box(&tree, "test").unwrap();
	let node = node.as_block_level_box();
	let lines = node.lines();
	assert_eq!(lines.len(), 2);
	let first_line = &lines[0];
	let second_line = &lines[1];
	assert_eq!(first_line.width(), Pixel::new(width1 + 10.0));
	assert_eq!(second_line.width(), Pixel::new(width2 + 10.0));
	assert_eq!(second_line.y(), Pixel::new(height1));
}

#[test]
#[serial]
fn block_box_contains_multilines_with_inline_block() {
	let tree = Rc::new(construct_tree(
		r#"
<div id="test" style="width: 150px;">
    <span style="margin: 0 10px">hello</span>
    <span style="display: inline-block; width: 200px">my friend</span>
</div>"#,
		r#""#,
	));
	let (width1, height1) = TextUI::new().measure_size("hello", &vec!["system-ui"], DEFAULT_FONT_SIZE);
	let node = find_box(&tree, "test").unwrap();
	let node = node.as_block_level_box();
	let lines = node.lines();
	assert_eq!(lines.len(), 2);
	let first_line = &lines[0];
	let second_line = &lines[1];
	assert_eq!(first_line.width(), Pixel::new(width1 + 20.0));
	assert_eq!(second_line.width(), Pixel::new(200.0));
	assert_eq!(second_line.y(), Pixel::new(height1));
}
