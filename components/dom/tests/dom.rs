use dom::parser::DomParser;
use html5ever::{
    driver,
    tendril::{StrTendril, TendrilSink},
};

#[test]
fn check_dom() {
    let sink = DomParser::default();

    let mut parser = driver::parse_document(sink, Default::default());
    parser.process(StrTendril::from(
        r#"
    <!DOCTYPE html>
    <div class="foo">Hello world!</div>
"#,
    ));

    let output = parser.finish();
    print!("{}", output);
}
