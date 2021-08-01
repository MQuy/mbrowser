use super::nonts_pseudo_class::NonTSPseudoClassFlag;
use cssparser::{
    BasicParseError, BasicParseErrorKind, CowRcStr, ParseError, Parser, SourceLocation, Token,
    _cssparser_internal_to_lowercase, match_ignore_ascii_case,
};
use html5ever::Namespace;
use selectors::parser::SelectorParseErrorKind;

use crate::str::starts_with_ignore_ascii_case;
use crate::stylesheets::origin::Origin;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::Namespaces;
use crate::values::CustomIdent;

use super::nonts_pseudo_class::NonTSPseudoClass;
use super::pseudo_element::PseudoElement;
use super::select_impl::SelectorImpl;

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

    fn is_pseudo_element_enabled(&self, pseudo_element: &PseudoElement) -> bool {
        if pseudo_element.enabled_in_content() {
            return true;
        }

        if self.in_user_agent_stylesheet() && pseudo_element.enabled_in_ua_sheets() {
            return true;
        }

        return false;
    }
}

impl<'a, 'i> ::selectors::Parser<'i> for SelectorParser<'a> {
    type Impl = SelectorImpl;
    type Error = StyleParseErrorKind<'i>;

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
        Err(
            location.new_custom_error(SelectorParseErrorKind::UnsupportedPseudoClassOrElement(
                name,
            )),
        )
    }

    fn parse_non_ts_functional_pseudo_class<'t>(
        &self,
        name: CowRcStr<'i>,
        parser: &mut Parser<'i, 't>,
    ) -> Result<NonTSPseudoClass, ParseError<'i, Self::Error>> {
        let pseudo_class = match_ignore_ascii_case! { &name,
            "lang" => {
                let name = parser.expect_ident_or_string()?;
                NonTSPseudoClass::Lang(CustomIdent::from(name.as_ref()))
            },
            _ => return Err(parser.new_custom_error(
                SelectorParseErrorKind::UnsupportedPseudoClassOrElement(name.clone())
            ))
        };
        if self.is_pseudo_class_enabled(&pseudo_class) {
            Ok(pseudo_class)
        } else {
            Err(
                parser.new_custom_error(SelectorParseErrorKind::UnsupportedPseudoClassOrElement(
                    name,
                )),
            )
        }
    }

    fn parse_pseudo_element(
        &self,
        location: SourceLocation,
        name: CowRcStr<'i>,
    ) -> Result<PseudoElement, ParseError<'i, Self::Error>> {
        let pseudo_element = match_ignore_ascii_case! { &name,
            "before" => PseudoElement::Before,
            "after" => PseudoElement::After,
            "selection" => PseudoElement::Selection,
            _ => return Err(location.new_custom_error(SelectorParseErrorKind::UnexpectedIdent(name.clone())))

        };

        Ok(pseudo_element)
    }

    fn parse_functional_pseudo_element<'t>(
        &self,
        name: CowRcStr<'i>,
        parser: &mut Parser<'i, 't>,
    ) -> Result<PseudoElement, ParseError<'i, Self::Error>> {
        todo!()
    }

    fn default_namespace(&self) -> Option<Namespace> {
        self.namespaces.default.clone()
    }
}
