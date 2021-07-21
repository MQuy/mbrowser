use std::rc::Weak;

use crate::{
    characterdata::CharacterData,
    document::Document,
    inheritance::{Castable, DerivedFrom},
    node::Node,
    nodetype::{CharacterDataTypeId, NodeTypeId},
};

#[derive(Clone)]
pub struct Comment {
    character_data: CharacterData,
}

impl Comment {
    pub fn new(text: String, document: Weak<Document>) -> Comment {
        Comment {
            character_data: CharacterData::new_inherited(
                NodeTypeId::CharacterData(CharacterDataTypeId::Comment),
                text,
                document,
            ),
        }
    }
}

impl Castable for Comment {}
impl DerivedFrom<CharacterData> for Comment {}
impl DerivedFrom<Node> for Comment {}
