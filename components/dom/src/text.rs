use std::rc::Weak;

use crate::{
    characterdata::CharacterData,
    document::Document,
    inheritance::{Castable, DerivedFrom},
    node::Node,
};

#[derive(Clone)]
pub struct Text {
    character_data: CharacterData,
}

impl Text {
    pub fn new(text: String, document: Weak<Document>) -> Self {
        Text {
            character_data: CharacterData::new(text, document),
        }
    }
}

impl Castable for Text {}
impl DerivedFrom<Node> for Text {}
impl DerivedFrom<CharacterData> for Text {}
