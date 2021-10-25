use core::fmt;
use std::fmt::Write;
use std::io::Cursor;

use cssparser::{
	CowRcStr, Parser, SourceLocation, ToCss, _cssparser_internal_to_lowercase,
	match_ignore_ascii_case,
};
use euclid::Length;
use html5ever::LocalName;
use murmur3::murmur3_32;
use precomputed_hash::PrecomputedHash;
use selectors::parser::SelectorParseErrorKind;

use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;

pub mod animation;
pub mod computed;
pub mod generics;
pub mod specified;
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

#[derive(Clone, Copy, Debug)]
pub enum CSSPixel {}

pub type Pixel = Length<f32, CSSPixel>;

pub const PIXEL_ZERO: Pixel = Pixel::new(0.0);

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Ident(pub String);

impl Ident {
	pub fn new(value: String) -> Self {
		Self(value)
	}
}
impl PartialEq<&str> for Ident {
	fn eq(&self, other: &&str) -> bool {
		self.0 == *other
	}
}

impl PartialEq<LocalName> for Ident {
	fn eq(&self, other: &LocalName) -> bool {
		self.0 == other.to_string()
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

impl PrecomputedHash for Ident {
	fn precomputed_hash(&self) -> u32 {
		murmur3_32(&mut Cursor::new(&self.0), 0).unwrap()
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CSSString(pub String);

impl ToCss for CSSString {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		cssparser::CssStringWriter::new(dest).write_str(&self.0)
	}
}

impl From<&str> for CSSString {
	fn from(value: &str) -> Self {
		Self(value.to_string())
	}
}

impl AsRef<str> for CSSString {
	fn as_ref(&self) -> &str {
		&*self.0
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
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		let location = input.current_source_location();
		let ident = input.expect_ident()?;
		CustomIdent::from_ident(location, ident, &[])
	}

	pub fn parse_excluding<'i, 't>(
		input: &mut Parser<'i, 't>,
		excluding: &[&str],
	) -> Result<Self, ParseError<'i>> {
		let location = input.current_source_location();
		let ident = input.expect_ident()?;
		CustomIdent::from_ident(location, ident, excluding)
	}

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

pub struct GenericAtomIdent<Set>(pub string_cache::Atom<Set>)
where
	Set: string_cache::StaticAtomSet;

impl<Set: string_cache::StaticAtomSet> Default for GenericAtomIdent<Set> {
	fn default() -> Self {
		Self(string_cache::Atom::default())
	}
}

impl<Set: string_cache::StaticAtomSet> std::fmt::Debug for GenericAtomIdent<Set> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl<Set: string_cache::StaticAtomSet> std::hash::Hash for GenericAtomIdent<Set> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.0.hash(state)
	}
}

impl<Set: string_cache::StaticAtomSet> Eq for GenericAtomIdent<Set> {}

impl<Set: string_cache::StaticAtomSet> PartialEq for GenericAtomIdent<Set> {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl<Set: string_cache::StaticAtomSet> Clone for GenericAtomIdent<Set> {
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl<Set: string_cache::StaticAtomSet> cssparser::ToCss for GenericAtomIdent<Set> {
	fn to_css<W>(&self, dest: &mut W) -> fmt::Result
	where
		W: Write,
	{
		dest.write_str(&self.0)
	}
}

impl<'a, Set: string_cache::StaticAtomSet> From<&'a str> for GenericAtomIdent<Set> {
	#[inline]
	fn from(string: &str) -> Self {
		Self(string_cache::Atom::from(string))
	}
}

impl<Set: string_cache::StaticAtomSet> std::borrow::Borrow<string_cache::Atom<Set>>
	for GenericAtomIdent<Set>
{
	#[inline]
	fn borrow(&self) -> &string_cache::Atom<Set> {
		&self.0
	}
}

impl<Set: string_cache::StaticAtomSet> PrecomputedHash for GenericAtomIdent<Set> {
	#[inline]
	fn precomputed_hash(&self) -> u32 {
		self.0.precomputed_hash()
	}
}

impl<Set: string_cache::StaticAtomSet> GenericAtomIdent<Set> {
	/// Constructs a new GenericAtomIdent.
	#[inline]
	pub fn new(atom: string_cache::Atom<Set>) -> Self {
		Self(atom)
	}

	/// Cast an atom ref to an AtomIdent ref.
	#[inline]
	pub fn cast<'a>(atom: &'a string_cache::Atom<Set>) -> &'a Self {
		let ptr = atom as *const _ as *const Self;
		// safety: repr(transparent)
		unsafe { &*ptr }
	}
}
