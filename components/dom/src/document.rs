use dom_struct::dom_struct;
use encoding_rs::Encoding;
use mime::Mime;

use crate::node::Node;

#[dom_struct]
pub struct Document {
    node: Node,
    content_type: Mime,
    encoding: &'static Encoding,
}
