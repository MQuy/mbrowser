use std::cell::Cell;

use encoding_rs::Encoding;
use html5ever::tree_builder::QuirksMode;
use mime::Mime;

use crate::{
    inheritance::{Castable, DerivedFrom},
    node::Node,
};

#[derive(Clone)]
pub struct Document {
    node: Node,
    content_type: Mime,
    encoding: &'static Encoding,
    quirk_mode: Cell<QuirksMode>,
}

impl Document {
    pub fn set_quirks_mode(&self, mode: QuirksMode) {
        self.quirk_mode.set(mode);
    }
}

impl Castable for Document {}
impl DerivedFrom<Node> for Document {}
