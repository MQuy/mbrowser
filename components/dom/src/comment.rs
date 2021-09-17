use std::rc::Rc;

use crate::characterdata::CharacterData;
use crate::document::Document;
use crate::global_scope::add_to_global_scope;
use crate::inheritance::{upcast, Castable, DerivedFrom};
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

	pub fn create(text: String, document: Rc<Document>) -> Rc<Self> {
		let comment = Rc::new(Self::new(text, document));
		add_to_global_scope(upcast(comment.clone()));
		comment
	}
}

impl Castable for Comment {}
impl DerivedFrom<CharacterData> for Comment {}
impl DerivedFrom<Node> for Comment {}
