use std::rc::Rc;

use crate::characterdata::CharacterData;
use crate::document::Document;
use crate::inheritance::{Castable, DerivedFrom};
use crate::node::Node;
use crate::nodetype::{CharacterDataTypeId, NodeTypeId, TextTypeId};

#[derive(Clone)]
#[repr(C)]
pub struct Text {
    character_data: CharacterData,
}

impl Text {
    pub fn new(text: String, document: Rc<Document>) -> Self {
        Text::new_inherited(
            NodeTypeId::CharacterData(CharacterDataTypeId::Text(TextTypeId::Text)),
            text,
            document,
        )
    }

    pub fn new_inherited(node_type_id: NodeTypeId, text: String, document: Rc<Document>) -> Self {
        Self {
            character_data: CharacterData::new_inherited(node_type_id, text, document),
        }
    }
}

impl Castable for Text {}
impl DerivedFrom<Node> for Text {}
impl DerivedFrom<CharacterData> for Text {}
