use html5ever::{LocalName, Namespace, Prefix};

use crate::node::Node;

pub struct Element {
    node: Node,
    prefix: Option<Prefix>,
    local_name: LocalName,
    tag_name: LocalName,
    namespace: Namespace,
}
