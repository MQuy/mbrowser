use setup::construct_tree;

mod setup;

#[test]
fn block_box_contains_inline_block_box() {
	let box_tree = construct_tree(
		r#"
<div style="color: red;">
    Hello world!
    <div id="hello" style="display: inline-block">
        <div>Echo from the past</div>
    </div>
    <p><span>Totoland</span></p>
</div>"#,
		r#"
#hello {
	align-content: normal;
}
    "#,
	);
	box_tree.log();
}
