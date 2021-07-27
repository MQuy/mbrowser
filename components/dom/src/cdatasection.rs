use std::rc::Rc;

use crate::characterdata::CharacterData;
use crate::document::Document;
use crate::node::Node;
use crate::nodetype::{CharacterDataTypeId, NodeTypeId, TextTypeId};
use crate::text::Text;

#[derive(Clone)]
#[repr(C)]
pub struct CDATASection {
    text: Text,
}

impl crate::inheritance::Castable for CDATASection {}
impl crate::inheritance::DerivedFrom<Node> for CDATASection {}
impl crate::inheritance::DerivedFrom<CharacterData> for CDATASection {}
impl crate::inheritance::DerivedFrom<Text> for CDATASection {}

impl CDATASection {
    pub fn new(text: String, document: Rc<Document>) -> Self {
        Self {
            text: Text::new_inherited(
                NodeTypeId::CharacterData(CharacterDataTypeId::Text(TextTypeId::CDATASection)),
                text,
                document,
            ),
        }
    }
}
