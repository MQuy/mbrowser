use std::sync::Arc;

use common::url::BrowserUrl;

#[derive(Clone)]
pub struct CssUrl {
    original: Option<Arc<String>>,
    resolved: Option<BrowserUrl>,
}
