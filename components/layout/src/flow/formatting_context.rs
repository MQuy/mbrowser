use std::cell::RefCell;
use std::rc::{Rc, Weak};

use super::boxes::VisualBox;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum FormattingContextType {
	BlockFormattingContext,
	InlineFormattingContext,
}

pub struct FormattingContext {
	pub formatting_context_type: FormattingContextType,
	pub established_by: RefCell<Option<Weak<VisualBox>>>,
}

impl FormattingContext {
	pub fn new(context_type: FormattingContextType) -> Self {
		FormattingContext {
			established_by: RefCell::new(None),
			formatting_context_type: context_type,
		}
	}

	pub fn set_established_by(&self, owner: Rc<VisualBox>) {
		self.established_by.replace(Some(Rc::downgrade(&owner)));
	}
}
