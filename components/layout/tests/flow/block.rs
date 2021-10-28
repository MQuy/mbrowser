use css::values::{Pixel, PIXEL_ZERO};
use dom::window;
use serial_test::serial;

use self::setup::{construct_tree, get_box_dimension};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
#[serial]
fn block_level_with_auto_width() {
	let tree = construct_tree(r#"<div id="test">hello world</div>"#, r#""#);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.width, Pixel::new(window::DEFAULT_WIDTH));
}

#[test]
#[serial]
fn block_level_with_auto_width_and_include_margin_padding() {
	let tree = construct_tree(
		r#"<div id="test">hello world</div>"#,
		r#"#test { padding: 100px; margin: 50px }"#,
	);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.width, Pixel::new(window::DEFAULT_WIDTH - 300.0));
}

#[test]
#[serial]
fn block_level_with_fixed_width() {
	let tree = construct_tree(
		r#"<div id="test">hello world</div>"#,
		r#"#test { width: 400px; }"#,
	);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.width, Pixel::new(400.0));
}

#[test]
#[serial]
fn block_level_with_percentage_width() {
	let tree = construct_tree(
		r#"<div id="test">hello world</div>"#,
		r#"#test { width: 60%; }"#,
	);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.width, Pixel::new(window::DEFAULT_WIDTH * 0.6));
}

#[test]
#[serial]
fn block_level_with_auto_width_margin_left_right_auto_to_zero() {
	let tree = construct_tree(
		r#"<div id="test"></div>"#,
		r#"
body { width: 400px; }
#test { width: auto; margin: 0 auto; }
        "#,
	);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.margin.margin_left, PIXEL_ZERO);
	assert_eq!(dimension.margin.margin_right, PIXEL_ZERO);
}

#[test]
#[serial]
fn block_level_with_non_auto_width_and_over_constrained_margin_left_right_auto_to_zero() {
	let tree = construct_tree(
		r#"<div id="test"></div>"#,
		r#"
body { width: 400px; }
#test { width: 360px; padding: 0 20px; margin: 0 auto; }
        "#,
	);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.margin.margin_left, PIXEL_ZERO);
	assert_eq!(dimension.margin.margin_right, PIXEL_ZERO);
}

#[test]
#[serial]
fn block_level_with_non_auto_width_and_margin_left_auto_to_leftover() {
	let tree = construct_tree(
		r#"<div id="test"></div>"#,
		r#"
body { width: 400px; }
#test { width: 360px; padding: 0 10px; margin-left: auto; }
        "#,
	);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.margin.margin_left, Pixel::new(20.0));
	assert_eq!(dimension.margin.margin_right, PIXEL_ZERO);
}

#[test]
#[serial]
fn block_level_with_non_auto_width_and_margin_left_right_auto_to_divided_equal() {
	let tree = construct_tree(
		r#"<div id="test"></div>"#,
		r#"
body { width: 400px; }
#test { width: 360px; padding: 0 10px; margin: 0 auto; }
        "#,
	);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.margin.margin_left, Pixel::new(10.0));
	assert_eq!(dimension.margin.margin_right, Pixel::new(10.0));
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
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.height, Pixel::new(100.0));
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
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.height, Pixel::new(150.0));
}

#[test]
#[serial]
fn block_level_with_fixed_height() {
	let tree = construct_tree(r#"<div id="test"></div>"#, r#"#test { height: 400px; }"#);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.height, Pixel::new(400.0));
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
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.height, Pixel::new(window::DEFAULT_HEIGHT * 0.4));
}

#[test]
#[serial]
fn block_level_with_margin_top_bottom_auto_to_zero() {
	let tree = construct_tree(
		r#"<div id="test"></div>"#,
		r#"#test { height: 400px; margin: auto 0 }"#,
	);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.height, Pixel::new(400.0));
	assert_eq!(dimension.margin.margin_top, PIXEL_ZERO);
	assert_eq!(dimension.margin.margin_bottom, PIXEL_ZERO);
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
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.margin.margin_top, Pixel::new(5.0));
	assert_eq!(dimension.margin.margin_right, Pixel::new(10.0));
	assert_eq!(dimension.margin.margin_bottom, Pixel::new(15.0));
	assert_eq!(dimension.margin.margin_left, Pixel::new(20.0));
	assert_eq!(dimension.padding.padding_top, Pixel::new(30.0));
	assert_eq!(dimension.padding.padding_right, Pixel::new(35.0));
	assert_eq!(dimension.padding.padding_bottom, Pixel::new(40.0));
	assert_eq!(dimension.padding.padding_left, Pixel::new(45.0));
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
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(
		dimension.margin.margin_top,
		Pixel::new(window::DEFAULT_WIDTH * 0.01)
	);
	assert_eq!(
		dimension.margin.margin_right,
		Pixel::new(window::DEFAULT_WIDTH * 0.02)
	);
	assert_eq!(
		dimension.margin.margin_bottom,
		Pixel::new(window::DEFAULT_WIDTH * 0.03)
	);
	assert_eq!(
		dimension.margin.margin_left,
		Pixel::new(window::DEFAULT_WIDTH * 0.04)
	);
	assert_eq!(
		dimension.padding.padding_top,
		Pixel::new(window::DEFAULT_WIDTH * 0.11)
	);
	assert_eq!(
		dimension.padding.padding_right,
		Pixel::new(window::DEFAULT_WIDTH * 0.12)
	);
	assert_eq!(
		dimension.padding.padding_bottom,
		Pixel::new(window::DEFAULT_WIDTH * 0.13)
	);
	assert_eq!(
		dimension.padding.padding_left,
		Pixel::new(window::DEFAULT_WIDTH * 0.14)
	);
}

#[test]
#[serial]
fn block_level_as_first_child_top_left_position() {
	let tree = construct_tree(r#"<div id="test">hello world</div>"#, r#""#);
	let dimension = get_box_dimension(&tree, "test").unwrap();
	assert_eq!(dimension.x, Pixel::new(0.0));
	assert_eq!(dimension.y, Pixel::new(0.0));
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
	let dimension = get_box_dimension(&tree, "test2").unwrap();
	assert_eq!(dimension.x, Pixel::new(0.0));
	assert_eq!(dimension.y, Pixel::new(90.0));
}
