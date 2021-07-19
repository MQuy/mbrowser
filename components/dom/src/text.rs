use dom_struct::dom_struct;

use crate::characterdata::CharacterData;

#[dom_struct]
pub struct Text {
    character_data: CharacterData,
}
