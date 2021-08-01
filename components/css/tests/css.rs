use std::rc::Rc;

use css::stylesheets::origin::Origin;
use css::stylesheets::stylesheet::{QuirksMode, Stylesheet};

#[test]
pub fn check_parser() {
    let css = r"
    @page {
        margin-left: 3cm;
    }
    @media (min-width: 10px invalid 1000px) {}
    @media screen { @invalid; }
    @supports (color: green) and invalid and (margin: 0) {}
    @keyframes foo { from invalid {} to { margin: 0 invalid 0; } }
    ";
    let media = Rc::new(vec![]);
    Stylesheet::from_str(css, Origin::UserAgent, media, QuirksMode::NoQuirks, 5);
}
