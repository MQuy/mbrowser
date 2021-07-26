use dom::parser::DomParser;
use html5ever::{
    parse_document,
    tendril::{StrTendril, TendrilSink},
};

#[test]
fn check_dom() {
    let sink = DomParser::default();

    let mut parser = parse_document(sink, Default::default());
    parser.process(StrTendril::from("<div>\n"));
    parser.process(StrTendril::from("</div>\n"));

    let output = parser.finish();
    print!("{}", output);
}
