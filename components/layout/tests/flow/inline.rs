use css::values::{Pixel, PIXEL_ZERO};
use dom::window;
use layout::text::TextUI;
use serial_test::serial;

use self::setup::{construct_tree, get_box_dimension};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
#[serial]
fn inline_level_with_auto_width_ignored() {
	let tree = construct_tree(r#"<span id="test">hello world</span>"#, r#""#);
	let (width, _) = TextUI::new().measure_size("hello world", &vec!["system-ui"], 14.0);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.width, Pixel::new(width));
}

#[test]
#[serial]
fn inline_level_with_fixed_width_ignored() {
	let tree = construct_tree(
		r#"<span id="test">hello world</span>"#,
		r#"#test { width: 400px; }"#,
	);
	let (width, _) = TextUI::new().measure_size("hello world", &vec!["system-ui"], 14.0);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.width, Pixel::new(width));
}

#[test]
#[serial]
fn inline_level_with_percentage_width_ignored() {
	let tree = construct_tree(
		r#"<span id="test">hello world</span>"#,
		r#"
#test { width: 400px; }
        "#,
	);
	let (width, _) = TextUI::new().measure_size("hello world", &vec!["system-ui"], 14.0);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.width, Pixel::new(width));
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
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.margin.margin_left, PIXEL_ZERO);
	assert_eq!(dimension.margin.margin_right, PIXEL_ZERO);
}

#[test]
#[serial]
fn inline_level_with_fixed_margin_padding_top_bottom_to_zero() {
	let tree = construct_tree(
		r#"<span id="test">hello world</span>"#,
		r#"
#test { margin: 10px 0 20px 0; padding: 5px 0 10px 0; }
        "#,
	);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.margin.margin_top, PIXEL_ZERO);
	assert_eq!(dimension.margin.margin_bottom, PIXEL_ZERO);
	assert_eq!(dimension.padding.padding_top, PIXEL_ZERO);
	assert_eq!(dimension.padding.padding_bottom, PIXEL_ZERO);
}

#[test]
#[serial]
fn inline_level_with_percentage_margin_padding_top_bottom_to_zero() {
	let tree = construct_tree(
		r#"<span id="test">hello world</span>"#,
		r#"
#test { margin: 1% 0 2% 0; padding: 11% 0 12% 0; }
        "#,
	);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.margin.margin_top, PIXEL_ZERO);
	assert_eq!(dimension.margin.margin_bottom, PIXEL_ZERO);
	assert_eq!(dimension.padding.padding_top, PIXEL_ZERO);
	assert_eq!(dimension.padding.padding_bottom, PIXEL_ZERO);
}

#[test]
#[serial]
fn inline_level_with_fixed_margin_padding_left_right() {
	let tree = construct_tree(
		r#"<span id="test">hello world</span>"#,
		r#"
#test { margin: 0 40px 0 20px; padding: 0 5px 0 10px; }
        "#,
	);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.margin.margin_left, Pixel::new(20.0));
	assert_eq!(dimension.margin.margin_right, Pixel::new(40.0));
	assert_eq!(dimension.padding.padding_left, Pixel::new(10.0));
	assert_eq!(dimension.padding.padding_right, Pixel::new(5.0));
}

#[test]
#[serial]
fn inline_level_with_percentage_margin_padding_left_right() {
	let tree = construct_tree(
		r#"<span id="test">hello world</span>"#,
		r#"
#test { margin: 0 10% 0 20%; padding: 0 5% 0 15%; }
        "#,
	);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(
		dimension.margin.margin_left,
		Pixel::new(window::DEFAULT_WIDTH * 0.2)
	);
	assert_eq!(
		dimension.margin.margin_right,
		Pixel::new(window::DEFAULT_WIDTH * 0.1)
	);
	assert_eq!(
		dimension.padding.padding_left,
		Pixel::new(window::DEFAULT_WIDTH * 0.15)
	);
	assert_eq!(
		dimension.padding.padding_right,
		Pixel::new(window::DEFAULT_WIDTH * 0.05)
	);
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
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.margin.margin_left, Pixel::new(0.0));
	assert_eq!(dimension.margin.margin_right, Pixel::new(0.0));
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
	let dimension = get_box_dimension(&tree, "test1").unwrap();
	assert_eq!(dimension.width, Pixel::new(800.0));
	let dimension1 = get_box_dimension(&tree, "test1").unwrap();
	assert_eq!(dimension1.width, Pixel::new(800.0));
}

#[test]
#[serial]
fn inline_level_as_first_child_top_left_position() {
	let tree = construct_tree(r#"<span id="test">hello world</span>"#, r#""#);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.x, Pixel::new(0.0));
	assert_eq!(dimension.y, Pixel::new(0.0));
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
	let (width, _) = TextUI::new().measure_size("hello world", &vec!["system-ui"], 14.0);
	let dimension = get_box_dimension(&tree, "test2").unwrap();
	assert_eq!(dimension.x, Pixel::new(width + 50.0));
	assert_eq!(dimension.y, Pixel::new(0.0));
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
	let dimension = get_box_dimension(&tree, "test2").unwrap();
	assert_eq!(dimension.x, Pixel::new(450.0));
	assert_eq!(dimension.y, Pixel::new(0.0));
}
