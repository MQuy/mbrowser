use core::fmt;
use std::fmt::Write;

use cssparser::{
    CowRcStr, SourceLocation, ToCss, _cssparser_internal_to_lowercase, match_ignore_ascii_case,
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

/// Whether to allow negative lengths or not.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub enum AllowedNumericType {
    /// Allow all kind of numeric values.
    All,
    /// Allow only non-negative numeric values.
    NonNegative,
}

impl AllowedNumericType {
    pub fn is_ok(&self, value: f32) -> bool {
        match self {
            AllowedNumericType::All => true,
            AllowedNumericType::NonNegative => value >= 0.0,
        }
    }
}

/// A CSS float value.
pub type CSSFloat = f32;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Ident(pub String);

impl PartialEq<&str> for Ident {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl From<&str> for Ident {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl ToCss for Ident {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        dest.write_str(&self.0)
    }
}

#[macro_export]
macro_rules! ident {
    ($arg:tt) => {
        Ident(String::from($arg))
    };
}

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

impl ToCss for CustomIdent {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: Write,
    {
        dest.write_str(&self.0)
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
