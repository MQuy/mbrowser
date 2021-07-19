use crate::characterdata::CharacterData;
use dom_struct::dom_struct;

#[dom_struct]
pub struct Comment {
    character_data: CharacterData,
}
