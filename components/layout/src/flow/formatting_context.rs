use core::panic;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use super::boxes::Box;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum FormattingContextType {
	BlockFormattingContext,
	InlineFormattingContext,
}

pub struct FormattingContext {
	pub formatting_context_type: FormattingContextType,
	pub established_by: RefCell<Option<Weak<dyn Box>>>,
}

impl FormattingContext {
	pub fn new(context_type: FormattingContextType) -> Self {
		FormattingContext {
			established_by: RefCell::new(None),
			formatting_context_type: context_type,
		}
	}

	pub fn set_established_by(&self, owner: Rc<dyn Box>) {
		self.established_by.replace(Some(Rc::downgrade(&owner)));
	}

	pub fn established_by(&self) -> Rc<dyn Box> {
		match self.established_by.borrow().as_ref() {
			Some(value) => value.upgrade().unwrap(),
			None => panic!("a box has to belongs to a formatting context"),
		}
	}
}
