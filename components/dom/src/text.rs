use std::rc::Weak;

use crate::{
    characterdata::CharacterData,
    document::Document,
    inheritance::{Castable, DerivedFrom},
    node::Node,
    nodetype::{CharacterDataTypeId, NodeTypeId, TextTypeId},
};

#[derive(Clone)]
pub struct Text {
    character_data: CharacterData,
}

impl Text {
    pub fn new(text: String, document: Weak<Document>) -> Self {
        Text::new_inherited(
            NodeTypeId::CharacterData(CharacterDataTypeId::Text(TextTypeId::Text)),
            text,
            document,
        )
    }

    pub fn new_inherited(node_type_id: NodeTypeId, text: String, document: Weak<Document>) -> Self {
        Self {
            character_data: CharacterData::new_inherited(node_type_id, text, document),
        }
    }
}

impl Castable for Text {}
impl DerivedFrom<Node> for Text {}
impl DerivedFrom<CharacterData> for Text {}
