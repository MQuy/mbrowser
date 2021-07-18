use encoding_rs::Encoding;
use mime::Mime;

use crate::node::Node;

pub struct Document {
    node: Node,
    content_type: Mime,
    encoding: Encoding,
}
