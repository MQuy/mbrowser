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

#[test]
pub fn check_parser() {
    let css = r"
    @page {
        margin-left: 3cm;
    }
    ";
    let error_reporter = TestingErrorReporter::new();
    let media = Rc::new(MediaList::empty());
    Stylesheet::from_str(
        css,
        Origin::UserAgent,
        media,
        Some(&error_reporter),
        QuirksMode::NoQuirks,
        5,
    );
}
