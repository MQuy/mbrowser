use setup::{assert_stylesheet, parse};

#[path = "../setup/mod.rs"]
mod setup;

#[test]
pub fn only_auto() {
	let css = r#"
.name {
	aspect-ratio: auto;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_stylesheet(&stylesheet, css);
}

#[test]
pub fn only_ratio() {
	let css = r#"
.name {
	aspect-ratio: 1 / 1;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_stylesheet(&stylesheet, css);
}

#[test]
pub fn auto_and_ratio() {
	let css = r#"
.name {
	aspect-ratio: auto 1 / 0;
}
    "#;
	let output = r#"
.name {
	aspect-ratio: auto 1 / 0;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_stylesheet(&stylesheet, output);
}

#[test]
pub fn ratio_and_auto() {
	let css = r#"
.name {
	aspect-ratio: 5.5 / 2.5 auto;
}
    "#;
	let output = r#"
.name {
	aspect-ratio: auto 5.5 / 2.5;
}
    "#;
	let (stylesheet, _) = parse(css);
	assert_stylesheet(&stylesheet, output);
}
