use super::url::CssUrl;

#[derive(Clone)]
pub enum Image {
    None,
    Url(CssUrl),
    // Gradient(Box<G>),
}
