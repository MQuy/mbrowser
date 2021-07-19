use crate::{
    document::Document,
    inheritance::{Castable, DerivedFrom},
    node::Node,
    nodetype::{CharacterDataTypeId, NodeTypeId},
};

#[derive(Clone)]
pub struct CharacterData {
    node: Node,
    data: String,
}

impl CharacterData {
    pub fn new(data: String, document: &Document) -> CharacterData {
        CharacterData {
            data,
            node: Node::new(
                NodeTypeId::CharacterData(CharacterDataTypeId::Comment),
                Some(document),
            ),
        }
    }
}

impl Castable for CharacterData {}
impl DerivedFrom<Node> for CharacterData {}
