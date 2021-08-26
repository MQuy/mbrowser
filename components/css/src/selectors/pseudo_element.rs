use std::fmt;

use cssparser::ToCss;

use super::select_impl::SelectorImpl;

/// A pseudo-element, both public and private.
///
/// NB: If you add to this list, be sure to update `each_simple_pseudo_element` too.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(usize)]
pub enum PseudoElement {
	After = 0,
	Before,
	Selection,
}

/// The count of all pseudo-elements.
pub const PSEUDO_COUNT: usize = PseudoElement::Selection as usize + 1;

impl ToCss for PseudoElement {
	fn to_css<W>(&self, dest: &mut W) -> fmt::Result
	where
		W: fmt::Write,
	{
		use self::PseudoElement::*;
		dest.write_str(match *self {
			After => "::after",
			Before => "::before",
			Selection => "::selection",
		})
	}
}

impl ::selectors::parser::PseudoElement for PseudoElement {
	type Impl = SelectorImpl;
}
