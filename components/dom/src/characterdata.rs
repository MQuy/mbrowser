use crate::node::Node;
use dom_struct::dom_struct;

#[dom_struct]
pub struct CharacterData {
    node: Node,
    data: String,
}
