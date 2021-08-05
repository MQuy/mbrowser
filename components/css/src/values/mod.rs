use core::fmt;
use std::fmt::Write;

use cssparser::{
    match_ignore_ascii_case, CowRcStr, SourceLocation, _cssparser_internal_to_lowercase,
};
use selectors::parser::SelectorParseErrorKind;

use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;

pub mod animation;
pub mod border;
pub mod color;
pub mod image;
pub mod layout;
pub mod length;
pub mod number;
pub mod percentage;
pub mod position;
pub mod text;
pub mod time;
pub mod url;

/// A CSS float value.
pub type CSSFloat = f32;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Ident(pub String);

///
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CustomIdent(pub String);

impl CustomIdent {
    /// Parse an already-tokenizer identifier
    pub fn from_ident<'i>(
        location: SourceLocation,
        ident: &CowRcStr<'i>,
        excluding: &[&str],
    ) -> Result<Self, ParseError<'i>> {
        let valid = match_ignore_ascii_case! { ident,
            "initial" | "inherit" | "unset" | "default" | "revert" => false,
            _ => true
        };
        if !valid {
            return Err(
                location.new_custom_error(SelectorParseErrorKind::UnexpectedIdent(ident.clone()))
            );
        }
        if excluding.iter().any(|s| ident.eq_ignore_ascii_case(s)) {
            Err(location.new_custom_error(StyleParseErrorKind::UnspecifiedError))
        } else {
            Ok(CustomIdent(ident.to_string()))
        }
    }
}

impl cssparser::ToCss for CustomIdent {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: Write,
    {
        todo!()
    }
}

impl From<&str> for CustomIdent {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl ToString for CustomIdent {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
