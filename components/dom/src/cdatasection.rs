use std::rc::Rc;

use crate::{
    characterdata::CharacterData,
    document::Document,
    node::Node,
    nodetype::{CharacterDataTypeId, NodeTypeId, TextTypeId},
    text::Text,
};

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
