use std::cell::RefCell;
use std::rc::Rc;

use css::error_reporting::{ContextualParseError, ParseErrorReporter};
use css::media_queries::media_list::MediaList;
use css::stylesheets::origin::Origin;
use css::stylesheets::stylesheet::{QuirksMode, Stylesheet};
use cssparser::SourceLocation;

#[derive(Debug)]
struct CSSError {
    pub line: u32,
    pub column: u32,
    pub message: String,
}

struct TestingErrorReporter {
    errors: RefCell<Vec<CSSError>>,
}

impl TestingErrorReporter {
    pub fn new() -> Self {
        TestingErrorReporter {
            errors: RefCell::new(Vec::new()),
        }
    }
}

impl ParseErrorReporter for TestingErrorReporter {
    fn report_error(&self, location: SourceLocation, error: ContextualParseError) {
        self.errors.borrow_mut().push(CSSError {
            line: location.line,
            column: location.column,
            message: error.to_string(),
        })
    }
}

pub fn parse(css: &str) -> Stylesheet {
    let error_reporter = TestingErrorReporter::new();
    let media = Rc::new(MediaList::empty());
    Stylesheet::from_str(
        css,
        Origin::UserAgent,
        media,
        Some(&error_reporter),
        QuirksMode::NoQuirks,
        5,
    )
}

#[test]
pub fn check_namespace() {
    let css = r#"
    @namespace toto "http://toto.example.org";
    @namespace "http://example.com/foo";
    "#;
    let stylesheet = parse(css);
    println!("{}", stylesheet);
}

#[test]
pub fn check_style() {
    let css = r#"
    div {
        background-color: red;
        display: invalid;
        background-image: linear-gradient(0deg, black, invalid, transparent);
        invalid: true;
    }
    "#;
    parse(css);
}

#[test]
pub fn check_media() {
    let css = "@media (width: 10px) {}";
    let stylesheet = parse(css);
    println!("{}", stylesheet);
}
#[test]
pub fn check_supports() {
    let css = "@supports (color: green) and invalid and (margin: 0) {}";
    parse(css);
}

#[test]
pub fn check_keyframes() {
    let css = "@keyframes foo { from {} to {} }";
    let stylesheet = parse(css);
    println!("{}", stylesheet);
}

#[test]
pub fn check_page() {
    let css = r#"
    @page {
        background-color: red;
        display: invalid;
        background-image: linear-gradient(0deg, black, invalid, transparent);
        invalid: true;
    }
    "#;
    parse(css);
}
