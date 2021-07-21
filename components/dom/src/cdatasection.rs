use std::rc::Weak;

use crate::{
    characterdata::CharacterData,
    document::Document,
    node::Node,
    nodetype::{CharacterDataTypeId, NodeTypeId, TextTypeId},
    text::Text,
};

pub struct CDATASection {
    text: Text,
}

impl crate::inheritance::Castable for CDATASection {}
impl crate::inheritance::DerivedFrom<Node> for CDATASection {}
impl crate::inheritance::DerivedFrom<CharacterData> for CDATASection {}
impl crate::inheritance::DerivedFrom<Text> for CDATASection {}

impl CDATASection {
    pub fn new(text: String, document: Weak<Document>) -> Self {
        Self {
            text: Text::new_inherited(
                NodeTypeId::CharacterData(CharacterDataTypeId::Text(TextTypeId::CDATASection)),
                text,
                document,
            ),
        }
    }
}
