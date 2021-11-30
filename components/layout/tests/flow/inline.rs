use css::properties::longhands::font_size::DEFAULT_FONT_SIZE;
use css::values::{Pixel, PIXEL_ZERO};
use dom::window;
use layout::flow::fragment::Fragment;
use layout::text::TextUI;
use serial_test::serial;

use self::setup::{construct_tree, find_box, get_layout_info};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
#[serial]
fn inline_level_with_auto_width_ignored() {
	let tree = construct_tree(r#"<span id="test">hello world</span>"#, r#""#);
	let (width, height) = TextUI::new().measure_size("hello world", &vec!["system-ui"], DEFAULT_FONT_SIZE);
	let node = find_box(&tree, "test").unwrap();
	let fragments = node.as_inline_level_box().fragments();
	assert_eq!(fragments.len(), 1);
	let fragment = fragments[0].borrow();
	assert_eq!(fragment.width(), Pixel::new(width));
	assert_eq!(fragment.height(), Pixel::new(height));
}

#[test]
#[serial]
fn inline_level_with_fixed_width_ignored() {
	let tree = construct_tree(r#"<span id="test">hello world</span>"#, r#"#test { width: 400px; }"#);
	let (width, height) = TextUI::new().measure_size("hello world", &vec!["system-ui"], DEFAULT_FONT_SIZE);
	let node = find_box(&tree, "test").unwrap();
	let fragments = node.as_inline_level_box().fragments();
	assert_eq!(fragments.len(), 1);
	let fragment = fragments[0].borrow();
	assert_eq!(fragment.width(), Pixel::new(width));
	assert_eq!(fragment.height(), Pixel::new(height));
}

#[test]
#[serial]
fn inline_level_with_percentage_width_ignored() {
	let tree = construct_tree(
		r#"<span id="test">hello world</span>"#,
		r#"
#test { width: 40%; }
        "#,
	);
	let (width, height) = TextUI::new().measure_size("hello world", &vec!["system-ui"], DEFAULT_FONT_SIZE);
	let node = find_box(&tree, "test").unwrap();
	let fragments = node.as_inline_level_box().fragments();
	assert_eq!(fragments.len(), 1);
	let fragment = fragments[0].borrow();
	assert_eq!(fragment.width(), Pixel::new(width));
	assert_eq!(fragment.height(), Pixel::new(height));
}

#[test]
#[serial]
fn inline_level_with_auto_margin_left_right_to_zero() {
	let tree = construct_tree(
		r#"<span id="test">hello world</span>"#,
		r#"
#test { margin: 0 auto; }
        "#,
	);
	let node = find_box(&tree, "test").unwrap();
	let fragments = node.as_inline_level_box().fragments();
	assert_eq!(fragments.len(), 1);
	let fragment = fragments[0].borrow();
	assert_eq!(fragment.margin.left, PIXEL_ZERO);
	assert_eq!(fragment.margin.right, PIXEL_ZERO);
}

#[test]
#[serial]
fn inline_level_with_fixed_margin_padding() {
	let tree = construct_tree(
		r#"<span id="test">hello world</span>"#,
		r#"
#test { margin: 10px 5px 20px 15px; padding: 25px 35px 30px 40px; }
        "#,
	);
	let node = find_box(&tree, "test").unwrap();
	let fragments = node.as_inline_level_box().fragments();
	assert_eq!(fragments.len(), 1);
	let fragment = fragments[0].borrow();
	assert_eq!(fragment.margin.top, Pixel::new(10.0));
	assert_eq!(fragment.margin.right, Pixel::new(5.0));
	assert_eq!(fragment.margin.bottom, Pixel::new(20.0));
	assert_eq!(fragment.margin.left, Pixel::new(15.0));
	assert_eq!(fragment.padding.top, Pixel::new(25.0));
	assert_eq!(fragment.padding.right, Pixel::new(35.0));
	assert_eq!(fragment.padding.bottom, Pixel::new(30.0));
	assert_eq!(fragment.padding.left, Pixel::new(40.0));
}

#[test]
#[serial]
fn inline_level_with_percentage_margin_padding() {
	let tree = construct_tree(
		r#"<span id="test">hello world</span>"#,
		r#"
#test { margin: 10% 5% 10% 5%; padding: 15% 20% 15% 20%; }
        "#,
	);
	let node = find_box(&tree, "test").unwrap();
	let fragments = node.as_inline_level_box().fragments();
	assert_eq!(fragments.len(), 1);
	let fragment = fragments[0].borrow();
	assert_eq!(fragment.margin.top, Pixel::new(window::DEFAULT_WIDTH * 0.1));
	assert_eq!(fragment.margin.right, Pixel::new(window::DEFAULT_WIDTH * 0.05));
	assert_eq!(fragment.margin.bottom, Pixel::new(window::DEFAULT_WIDTH * 0.1));
	assert_eq!(fragment.margin.left, Pixel::new(window::DEFAULT_WIDTH * 0.05));
	assert_eq!(fragment.padding.top, Pixel::new(window::DEFAULT_WIDTH * 0.15));
	assert_eq!(fragment.padding.right, Pixel::new(window::DEFAULT_WIDTH * 0.20));
	assert_eq!(fragment.padding.bottom, Pixel::new(window::DEFAULT_WIDTH * 0.15));
	assert_eq!(fragment.padding.left, Pixel::new(window::DEFAULT_WIDTH * 0.20));
}

#[test]
#[serial]
fn inline_block_level_with_percentage_margin_left_right_to_zero() {
	let tree = construct_tree(
		r#"<div id="test"></div>"#,
		r#"
#test { display: inline-block; margin: 0px auto }
        "#,
	);
	let node = find_box(&tree, "test").unwrap();
	let fragments = node.as_inline_level_box().fragments();
	assert_eq!(fragments.len(), 1);
	let fragment = fragments[0].borrow();
	assert_eq!(fragment.margin.left, Pixel::new(0.0));
	assert_eq!(fragment.margin.right, Pixel::new(0.0));
}

#[test]
#[serial]
fn inline_block_level_with_over_width() {
	let tree = construct_tree(
		r#"<div id="test"><div id="test1"></div></div>"#,
		r#"
#test { display: inline; }
#test1 { display: inline-block; width: 800px; }
        "#,
	);
	let node = find_box(&tree, "test").unwrap();
	let fragments = node.as_inline_level_box().fragments();
	assert_eq!(fragments.len(), 1);
	let fragment = fragments[0].borrow();
	assert_eq!(fragment.width(), Pixel::new(800.0));
	let node1 = find_box(&tree, "test1").unwrap();
	let fragments1 = node1.as_inline_level_box().fragments();
	assert_eq!(fragments1.len(), 1);
	let fragment1 = fragments1[0].borrow();
	assert_eq!(fragment1.width(), Pixel::new(800.0));
}

#[test]
#[serial]
fn inline_level_as_first_child_top_left_position() {
	let tree = construct_tree(r#"<span id="test">hello world</span>"#, r#""#);
	let node = find_box(&tree, "test").unwrap();
	let fragments = node.as_inline_level_box().fragments();
	assert_eq!(fragments.len(), 1);
	let fragment = fragments[0].borrow();
	assert_eq!(fragment.x(), Pixel::new(0.0));
	assert_eq!(fragment.y(), Pixel::new(0.0));
}

#[test]
#[serial]
fn inline_level_as_second_child_position() {
	let tree = construct_tree(
		r#"
<span id="test1">hello world</span>
<span id="test2">hello world</span>
        "#,
		r#"
#test1 { padding: 10px; margin: 15px }
        "#,
	);
	let (width, _) = TextUI::new().measure_size("hello world", &vec!["system-ui"], DEFAULT_FONT_SIZE);
	let node = find_box(&tree, "test2").unwrap();
	let fragments = node.as_inline_level_box().fragments();
	assert_eq!(fragments.len(), 1);
	let fragment = fragments[0].borrow();
	assert_eq!(fragment.x(), Pixel::new(width + 50.0));
	assert_eq!(fragment.y(), Pixel::new(0.0));
}

#[test]
#[serial]
fn inline_block_level_as_second_child_position() {
	let tree = construct_tree(
		r#"
<span id="test1">hello world</span>
<span id="test2">hello world</span>
        "#,
		r#"
#test1 { display: inline-block; width: 400px; padding: 10px; margin: 15px }
        "#,
	);
	let node = find_box(&tree, "test2").unwrap();
	let fragments = node.as_inline_level_box().fragments();
	assert_eq!(fragments.len(), 1);
	let fragment = fragments[0].borrow();
	assert_eq!(fragment.x(), Pixel::new(450.0));
	assert_eq!(fragment.y(), Pixel::new(0.0));
}
