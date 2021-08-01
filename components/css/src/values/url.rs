use std::sync::Arc;

use common::url::BrowserUrl;

#[derive(Clone, Debug)]
pub struct CssUrl {
    original: Option<Arc<String>>,
    resolved: Option<BrowserUrl>,
}
