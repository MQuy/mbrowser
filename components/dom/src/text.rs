use crate::{
    characterdata::CharacterData,
    inheritance::{Castable, DerivedFrom},
    node::Node,
};

#[derive(Clone)]
pub struct Text {
    character_data: CharacterData,
}

impl Castable for Text {}
impl DerivedFrom<Node> for Text {}
impl DerivedFrom<CharacterData> for Text {}
