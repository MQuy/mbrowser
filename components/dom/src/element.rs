use html5ever::{LocalName, Namespace, Prefix};

use crate::node::Node;

use dom_struct::dom_struct;

#[dom_struct]
pub struct Element {
    node: Node,
    prefix: Option<Prefix>,
    local_name: LocalName,
    tag_name: LocalName,
    namespace: Namespace,
}
