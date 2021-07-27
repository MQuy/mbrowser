use std::rc::Rc;

use crate::characterdata::CharacterData;
use crate::document::Document;
use crate::inheritance::{Castable, DerivedFrom};
use crate::node::Node;
use crate::nodetype::{CharacterDataTypeId, NodeTypeId};

#[derive(Clone)]
#[repr(C)]
pub struct Comment {
    character_data: CharacterData,
}

impl Comment {
    pub fn new(text: String, document: Rc<Document>) -> Comment {
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
