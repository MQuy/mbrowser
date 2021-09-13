use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

use common::url::BrowserUrl;
use css::values::length::Length;
use css::values::CSSString;
use cssparser::{Color, RGBA};
use html5ever::{LocalName, Namespace, Prefix};
use num_traits::ToPrimitive;
use selectors::attr::AttrSelectorOperation;

use crate::element::Element;
use crate::str::{read_numbers, split_html_space_chars, HTML_SPACE_CHARACTERS};

const UNSIGNED_LONG_MAX: u32 = 2147483647;
// https://dom.spec.whatwg.org/#interface-attr
#[derive(Debug)]
pub struct Attr {
	local_name: LocalName,
	name: LocalName,
	namespace: Namespace,
	prefix: Option<Prefix>,
	value: RefCell<AttrValue>,
	owner: Weak<Element>,
}
impl Attr {
	pub fn new(
		local_name: LocalName,
		value: AttrValue,
		name: LocalName,
		namespace: Namespace,
		prefix: Option<Prefix>,
		owner: Rc<Element>,
	) -> Self {
		Attr {
			local_name,
			name,
			namespace,
			prefix,
			value: RefCell::new(value),
			owner: Rc::downgrade(&owner),
		}
	}

	pub fn get_owner(&self) -> Option<Rc<Element>> {
		self.owner.upgrade()
	}

	#[inline]
	pub fn get_name(&self) -> String {
		self.name.to_string()
	}

	#[inline]
	pub fn get_value(&self) -> String {
		String::from(&**self.value.borrow())
	}

	pub fn set_value(&self, value: AttrValue) {
		self.value.replace(value);
	}

	#[inline]
	pub fn value(&self) -> Ref<AttrValue> {
		self.value.borrow()
	}

	#[inline]
	pub fn get_local_name(&self) -> &LocalName {
		&self.local_name
	}

	#[inline]
	pub fn get_prefix(&self) -> Option<&Prefix> {
		match self.prefix {
			Some(ref prefix) => Some(prefix),
			None => None,
		}
	}

	#[inline]
	pub fn get_namespace(&self) -> &Namespace {
		&self.namespace
	}

	#[inline]
	pub fn as_tokens(&self) -> Option<Vec<String>> {
		match *self.value() {
			AttrValue::TokenList(_, ref tokens) => Some(tokens.clone()),
			_ => None,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LengthOrPercentageOrAuto {
	Auto,
	Percentage(f32),
	Length(f32),
}

#[derive(Clone, Debug)]
pub enum AttrValue {
	String(String),
	TokenList(String, Vec<String>),
	UInt(String, u32),
	Int(String, i32),
	Double(String, f64),
	Length(String, Option<Length>),
	Color(String, Option<RGBA>),
	Dimension(String, LengthOrPercentageOrAuto),

	/// Stores a URL, computed from the input string and a document's base URL.
	///
	/// The URL is resolved at setting-time, so this kind of attribute value is
	/// not actually suitable for most URL-reflecting IDL attributes.
	ResolvedUrl(String, Option<BrowserUrl>),
}

impl AttrValue {
	pub fn from_serialized_tokenlist(tokens: String) -> AttrValue {
		let atoms =
			split_html_space_chars(&tokens)
				.map(String::from)
				.fold(vec![], |mut acc, atom| {
					if !acc.contains(&atom) {
						acc.push(atom)
					}
					acc
				});
		AttrValue::TokenList(tokens, atoms)
	}

	pub fn from_resolved_url(base: &BrowserUrl, url: String) -> AttrValue {
		let joined = base.join(&url).ok();
		AttrValue::ResolvedUrl(url, joined)
	}

	pub fn from_legacy_color(string: String) -> AttrValue {
		let parsed = parse_legacy_color(&string).ok();
		AttrValue::Color(string, parsed)
	}

	// https://html.spec.whatwg.org/multipage/#reflecting-content-attributes-in-idl-attributes:idl-unsigned-long
	pub fn from_u32(string: String, default: u32) -> AttrValue {
		let result = parse_unsigned_integer(string.chars()).unwrap_or(default);
		let result = if result > UNSIGNED_LONG_MAX {
			default
		} else {
			result
		};
		AttrValue::UInt(string, result)
	}

	pub fn eval_selector(&self, selector: &AttrSelectorOperation<&CSSString>) -> bool {
		// FIXME(SimonSapin) this can be more efficient by matching on `(self, selector)` variants
		// and doing Atom comparisons instead of string comparisons where possible,
		// with SelectorImpl::AttrValue changed to Atom.
		selector.eval_str(self)
	}
}

impl ::std::ops::Deref for AttrValue {
	type Target = str;

	fn deref(&self) -> &str {
		match *self {
			AttrValue::String(ref value)
			| AttrValue::TokenList(ref value, _)
			| AttrValue::UInt(ref value, _)
			| AttrValue::Double(ref value, _)
			| AttrValue::Length(ref value, _)
			| AttrValue::Color(ref value, _)
			| AttrValue::Int(ref value, _)
			| AttrValue::ResolvedUrl(ref value, _)
			| AttrValue::Dimension(ref value, _) => &value,
		}
	}
}

/// Parses a [legacy color][color]. If unparseable, `Err` is returned.
///
/// [color]: https://html.spec.whatwg.org/multipage/#rules-for-parsing-a-legacy-colour-value
pub fn parse_legacy_color(mut input: &str) -> Result<RGBA, ()> {
	// Steps 1 and 2.
	if input.is_empty() {
		return Err(());
	}

	// Step 3.
	input = input.trim_matches(HTML_SPACE_CHARACTERS);

	// Step 4.
	if input.eq_ignore_ascii_case("transparent") {
		return Err(());
	}

	// Step 5.
	if let Ok(Color::RGBA(rgba)) = cssparser::parse_color_keyword(input) {
		return Ok(rgba);
	}

	// Step 6.
	if input.len() == 4 {
		if let (b'#', Ok(r), Ok(g), Ok(b)) = (
			input.as_bytes()[0],
			hex(input.as_bytes()[1] as char),
			hex(input.as_bytes()[2] as char),
			hex(input.as_bytes()[3] as char),
		) {
			return Ok(RGBA::new(r * 17, g * 17, b * 17, 255));
		}
	}

	// Step 7.
	let mut new_input = String::new();
	for ch in input.chars() {
		if ch as u32 > 0xffff {
			new_input.push_str("00")
		} else {
			new_input.push(ch)
		}
	}
	let mut input = &*new_input;

	// Step 8.
	for (char_count, (index, _)) in input.char_indices().enumerate() {
		if char_count == 128 {
			input = &input[..index];
			break;
		}
	}

	// Step 9.
	if input.as_bytes()[0] == b'#' {
		input = &input[1..]
	}

	// Step 10.
	let mut new_input = Vec::new();
	for ch in input.chars() {
		if hex(ch).is_ok() {
			new_input.push(ch as u8)
		} else {
			new_input.push(b'0')
		}
	}
	let mut input = new_input;

	// Step 11.
	while input.is_empty() || (input.len() % 3) != 0 {
		input.push(b'0')
	}

	// Step 12.
	let mut length = input.len() / 3;
	let (mut red, mut green, mut blue) = (
		&input[..length],
		&input[length..length * 2],
		&input[length * 2..],
	);

	// Step 13.
	if length > 8 {
		red = &red[length - 8..];
		green = &green[length - 8..];
		blue = &blue[length - 8..];
		length = 8
	}

	// Step 14.
	while length > 2 && red[0] == b'0' && green[0] == b'0' && blue[0] == b'0' {
		red = &red[1..];
		green = &green[1..];
		blue = &blue[1..];
		length -= 1
	}

	// Steps 15-20.
	return Ok(RGBA::new(
		hex_string(red).unwrap(),
		hex_string(green).unwrap(),
		hex_string(blue).unwrap(),
		255,
	));

	fn hex(ch: char) -> Result<u8, ()> {
		match ch {
			'0'..='9' => Ok((ch as u8) - b'0'),
			'a'..='f' => Ok((ch as u8) - b'a' + 10),
			'A'..='F' => Ok((ch as u8) - b'A' + 10),
			_ => Err(()),
		}
	}

	fn hex_string(string: &[u8]) -> Result<u8, ()> {
		match string.len() {
			0 => Err(()),
			1 => hex(string[0] as char),
			_ => {
				let upper = hex(string[0] as char)?;
				let lower = hex(string[1] as char)?;
				Ok((upper << 4) | lower)
			},
		}
	}
}

/// Parse an integer according to
/// <https://html.spec.whatwg.org/multipage/#rules-for-parsing-non-negative-integers>
pub fn parse_unsigned_integer<T: Iterator<Item = char>>(input: T) -> Result<u32, ()> {
	do_parse_integer(input).and_then(|result| result.to_u32().ok_or(()))
}

/// Shared implementation to parse an integer according to
/// <https://html.spec.whatwg.org/multipage/#rules-for-parsing-integers> or
/// <https://html.spec.whatwg.org/multipage/#rules-for-parsing-non-negative-integers>
fn do_parse_integer<T: Iterator<Item = char>>(input: T) -> Result<i64, ()> {
	let mut input = input
		.skip_while(|c| HTML_SPACE_CHARACTERS.iter().any(|s| s == c))
		.peekable();

	let sign = match input.peek() {
		None => return Err(()),
		Some(&'-') => {
			input.next();
			-1
		},
		Some(&'+') => {
			input.next();
			1
		},
		Some(_) => 1,
	};

	let (value, _) = read_numbers(input);

	value.and_then(|value| value.checked_mul(sign)).ok_or(())
}
