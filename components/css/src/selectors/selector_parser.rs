use cssparser::{CowRcStr, ParseError, SourceLocation};
use selectors::parser::SelectorParseErrorKind;

use super::nonts_pseudo_class::{NonTSPseudoClass, NonTSPseudoClassFlag};
use super::pseudo_element::PseudoElement;
use super::select::Selectors;
use crate::stylesheets::origin::Origin;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::Namespaces;
use crate::Namespace;

pub struct SelectorParser<'a> {
	/// The origin of the stylesheet we're parsing.
	pub stylesheet_origin: Origin,
	/// The namespace set of the stylesheet.
	pub namespaces: &'a Namespaces,
}

impl<'a> SelectorParser<'a> {
	/// Whether we're parsing selectors in a user-agent stylesheet.
	pub fn in_user_agent_stylesheet(&self) -> bool {
		matches!(self.stylesheet_origin, Origin::UserAgent)
	}

	fn is_pseudo_class_enabled(&self, pseudo_class: &NonTSPseudoClass) -> bool {
		if pseudo_class.is_enabled_in_content() {
			return true;
		}

		if self.in_user_agent_stylesheet()
			&& pseudo_class.has_any_flag(NonTSPseudoClassFlag::PSEUDO_CLASS_ENABLED_IN_UA_SHEETS)
		{
			return true;
		}

		return false;
	}

	fn is_pseudo_element_enabled(&self, _pseudo_element: &PseudoElement) -> bool {
		if self.in_user_agent_stylesheet() {
			return true;
		}

		return false;
	}
}

impl<'a, 'i> ::selectors::Parser<'i> for SelectorParser<'a> {
	type Error = StyleParseErrorKind<'i>;
	type Impl = Selectors;

	#[inline]
	fn parse_slotted(&self) -> bool {
		true
	}

	#[inline]
	fn parse_host(&self) -> bool {
		true
	}

	#[inline]
	fn parse_is_and_where(&self) -> bool {
		true
	}

	#[inline]
	fn parse_part(&self) -> bool {
		true
	}

	fn parse_non_ts_pseudo_class(
		&self,
		location: SourceLocation,
		name: CowRcStr<'i>,
	) -> Result<NonTSPseudoClass, ParseError<'i, Self::Error>> {
		if let Some(pseudo_class) = NonTSPseudoClass::parse_non_functional(&name) {
			if self.is_pseudo_class_enabled(&pseudo_class) {
				return Ok(pseudo_class);
			}
		}
		Err(location.new_custom_error(SelectorParseErrorKind::UnsupportedPseudoClassOrElement(name)))
	}

	fn parse_pseudo_element(
		&self,
		location: SourceLocation,
		name: CowRcStr<'i>,
	) -> Result<PseudoElement, ParseError<'i, Self::Error>> {
		if let Some(pseudo) = PseudoElement::from_slice(&name) {
			if self.is_pseudo_element_enabled(&pseudo) {
				return Ok(pseudo);
			}
		}

		Err(location.new_custom_error(SelectorParseErrorKind::UnsupportedPseudoClassOrElement(name)))
	}

	fn default_namespace(&self) -> Option<Namespace> {
		self.namespaces.default.clone()
	}
}
