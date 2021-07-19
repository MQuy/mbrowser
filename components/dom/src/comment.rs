use crate::{
    characterdata::CharacterData,
    document::Document,
    inheritance::{Castable, DerivedFrom},
    node::Node,
};

#[derive(Clone)]
pub struct Comment {
    character_data: CharacterData,
}

impl Comment {
    pub fn new(text: String, document: &Document) -> Comment {
        Comment {
            character_data: CharacterData::new(text, document),
        }
    }
}

impl Castable for Comment {}
impl DerivedFrom<CharacterData> for Comment {}
impl DerivedFrom<Node> for Comment {}
