use std::rc::Rc;

use css::media_queries::media_list::MediaList;
use css::stylesheets::origin::Origin;
use css::stylesheets::stylesheet::{QuirksMode, Stylesheet};

#[test]
pub fn check_parser() {
    let css = r"
    @page {
        margin-left: 3cm;
    }
    ";
    let media = Rc::new(MediaList::empty());
    Stylesheet::from_str(css, Origin::UserAgent, media, QuirksMode::NoQuirks, 5);
}
