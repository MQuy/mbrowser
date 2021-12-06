use cssparser::{Parser, ToCss, Token, _cssparser_internal_to_lowercase, match_ignore_ascii_case};

use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;

/// Generic for Length/Auto
#[derive(Clone, Debug)]
pub enum GenericLengthOrAuto<Length> {
	Length(Length),
	Auto,
}

impl<L> GenericLengthOrAuto<L> {
	pub fn parse_with<'i, 't, LP>(input: &mut Parser<'i, 't>, length_parser: LP) -> Result<Self, ParseError<'i>>
	where
		LP: FnOnce(&mut Parser<'i, 't>) -> Result<L, ParseError<'i>>,
	{
		input
			.try_parse(|input| {
				input.expect_ident_matching("auto")?;
				Ok(Self::Auto)
			})
			.or_else(|_err: ParseError<'i>| {
				let length = length_parser(input)?;
				Ok(Self::Length(length))
			})
	}
}

impl<L: ToCss> ToCss for GenericLengthOrAuto<L> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			GenericLengthOrAuto::Length(value) => value.to_css(dest),
			GenericLengthOrAuto::Auto => dest.write_str("auto"),
		}
	}
}

/// Generic for Length/None
#[derive(Clone, Debug)]
pub enum GenericLengthOrNone<Length> {
	Length(Length),
	None,
}

impl<L> GenericLengthOrNone<L> {
	pub fn parse_with<'i, 't, LP>(input: &mut Parser<'i, 't>, length_parser: LP) -> Result<Self, ParseError<'i>>
	where
		LP: FnOnce(&mut Parser<'i, 't>) -> Result<L, ParseError<'i>>,
	{
		input
			.try_parse(|input| {
				input.expect_ident_matching("none")?;
				Ok(Self::None)
			})
			.or_else(|_err: ParseError<'i>| {
				let length = length_parser(input)?;
				Ok(Self::Length(length))
			})
	}
}

impl<L: ToCss> ToCss for GenericLengthOrNone<L> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			GenericLengthOrNone::Length(length) => length.to_css(dest),
			GenericLengthOrNone::None => dest.write_str("none"),
		}
	}
}

/// Generic for Length/Percentage/Auto
#[derive(Clone, Debug, PartialEq)]
pub enum GenericLengthPercentageOrAuto<LengthPercent> {
	LengthPercentage(LengthPercent),
	Auto,
}

impl<LP> GenericLengthPercentageOrAuto<LP> {
	pub fn parse_with<'i, 't, LPP>(
		input: &mut Parser<'i, 't>,
		length_percentage_parser: LPP,
	) -> Result<Self, ParseError<'i>>
	where
		LPP: FnOnce(&mut Parser<'i, 't>) -> Result<LP, ParseError<'i>>,
	{
		input
			.try_parse(|input| {
				input.expect_ident_matching("auto")?;
				Ok(Self::Auto)
			})
			.or_else(|_err: ParseError<'i>| {
				let length_percentage = length_percentage_parser(input)?;
				Ok(Self::LengthPercentage(length_percentage))
			})
	}
}

impl<LP: ToCss> ToCss for GenericLengthPercentageOrAuto<LP> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			GenericLengthPercentageOrAuto::LengthPercentage(value) => value.to_css(dest),
			GenericLengthPercentageOrAuto::Auto => dest.write_str("auto"),
		}
	}
}

/// Generic for Length/Percentage/Number/Auto
#[derive(Clone, Debug)]
pub enum GenericLengthPercentageNumberOrAuto<LengthPercent, Number> {
	LengthPercentage(LengthPercent),
	Number(Number),
	Auto,
}

impl<LP, N> GenericLengthPercentageNumberOrAuto<LP, N> {
	pub fn parse_with<'i, 't, LPP, NP>(
		input: &mut Parser<'i, 't>,
		length_percentage_parser: LPP,
		number_parser: NP,
	) -> Result<Self, ParseError<'i>>
	where
		LPP: FnOnce(&mut Parser<'i, 't>) -> Result<LP, ParseError<'i>>,
		NP: FnOnce(&mut Parser<'i, 't>) -> Result<N, ParseError<'i>>,
	{
		input
			.try_parse(|input| {
				input.expect_ident_matching("auto")?;
				Ok(Self::Auto)
			})
			.or_else(|_err: ParseError<'i>| {
				input
					.try_parse(|input| {
						let number = number_parser(input)?;
						Ok(Self::Number(number))
					})
					.or_else(|_err: ParseError<'i>| {
						let length_percentage = length_percentage_parser(input)?;
						Ok(Self::LengthPercentage(length_percentage))
					})
			})
	}
}

impl<LP: ToCss, N: ToCss> ToCss for GenericLengthPercentageNumberOrAuto<LP, N> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			GenericLengthPercentageNumberOrAuto::LengthPercentage(value) => value.to_css(dest),
			GenericLengthPercentageNumberOrAuto::Number(value) => value.to_css(dest),
			GenericLengthPercentageNumberOrAuto::Auto => dest.write_str("auto"),
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExtremumLength<LengthPercent> {
	MaxContent,
	MinContent,
	FitContent(LengthPercent),
}

impl<LP> ExtremumLength<LP> {
	pub fn parse_with<'i, 't, F>(input: &mut Parser<'i, 't>, item_parser: F) -> Result<Self, ParseError<'i>>
	where
		F: for<'ii, 'tt> Fn(&mut Parser<'ii, 'tt>) -> Result<LP, ParseError<'ii>>,
	{
		let location = input.current_source_location();
		let token = input.next()?.clone();
		match &token {
			Token::Ident(ident) => Ok(match_ignore_ascii_case! { ident,
				"max-content" => ExtremumLength::MaxContent,
				"min-content" => ExtremumLength::MinContent,
				_ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
			}),
			Token::Function(name) if name.eq_ignore_ascii_case("fit-content") => input.parse_nested_block(|input| {
				let length_percentage = item_parser(input)?;
				Ok(ExtremumLength::FitContent(length_percentage))
			}),
			_ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedToken(token.clone()))),
		}
	}
}

impl<LP: ToCss> ToCss for ExtremumLength<LP> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			ExtremumLength::MaxContent => dest.write_str("max-content"),
			ExtremumLength::MinContent => dest.write_str("min-content"),
			ExtremumLength::FitContent(value) => dest.write_fmt(format_args!("fit-content({})", value.to_css_string())),
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum GenericSize<LengthPercent> {
	Auto,
	LengthPercentage(LengthPercent),
	ExtremumLength(ExtremumLength<LengthPercent>),
}

impl<LP> GenericSize<LP> {
	pub fn parse_with<'i, 't, F>(input: &mut Parser<'i, 't>, item_parser: F) -> Result<Self, ParseError<'i>>
	where
		F: for<'ii, 'tt> Fn(&mut Parser<'ii, 'tt>) -> Result<LP, ParseError<'ii>>,
	{
		input
			.try_parse(|input| {
				input.expect_ident_matching("auto")?;
				Ok(Self::Auto)
			})
			.or_else(|_err: ParseError<'i>| {
				input
					.try_parse(|input| {
						let length_percentage = item_parser(input)?;
						Ok(Self::LengthPercentage(length_percentage))
					})
					.or_else(|_err: ParseError<'i>| {
						let extremum_length = ExtremumLength::parse_with(input, item_parser)?;
						Ok(Self::ExtremumLength(extremum_length))
					})
			})
	}
}

impl<LP: ToCss> ToCss for GenericSize<LP> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			GenericSize::Auto => dest.write_str("auto"),
			GenericSize::LengthPercentage(value) => value.to_css(dest),
			GenericSize::ExtremumLength(value) => value.to_css(dest),
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum GenericMaxSize<LengthPercent> {
	None,
	LengthPercentage(LengthPercent),
	ExtremumLength(ExtremumLength<LengthPercent>),
}

impl<LP> GenericMaxSize<LP> {
	pub fn parse_with<'i, 't, F>(input: &mut Parser<'i, 't>, item_parser: F) -> Result<Self, ParseError<'i>>
	where
		F: for<'ii, 'tt> Fn(&mut Parser<'ii, 'tt>) -> Result<LP, ParseError<'ii>>,
	{
		input
			.try_parse(|input| {
				input.expect_ident_matching("none")?;
				Ok(Self::None)
			})
			.or_else(|_err: ParseError<'i>| {
				input
					.try_parse(|input| {
						let length_percentage = item_parser(input)?;
						Ok(Self::LengthPercentage(length_percentage))
					})
					.or_else(|_err: ParseError<'i>| {
						let extremum_length = ExtremumLength::parse_with(input, item_parser)?;
						Ok(Self::ExtremumLength(extremum_length))
					})
			})
	}
}

impl<LP: ToCss> ToCss for GenericMaxSize<LP> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			Self::None => dest.write_str("none"),
			Self::LengthPercentage(value) => value.to_css(dest),
			Self::ExtremumLength(value) => value.to_css(dest),
		}
	}
}

#[derive(Clone, Debug)]
#[repr(C, u8)]
pub enum GenericLengthPercentageOrNormal<LengthPercent> {
	LengthPercentage(LengthPercent),
	Normal,
}

impl<LP> GenericLengthPercentageOrNormal<LP> {
	pub fn parse_with<'i, 't, F>(input: &mut Parser<'i, 't>, item_parser: F) -> Result<Self, ParseError<'i>>
	where
		F: Fn(&mut Parser<'i, 't>) -> Result<LP, ParseError<'i>>,
	{
		input
			.try_parse(|input| {
				input.expect_ident_matching("normal")?;
				Ok(Self::Normal)
			})
			.or_else(|_err: ParseError<'i>| {
				let length_percent = item_parser(input)?;
				Ok(Self::LengthPercentage(length_percent))
			})
	}
}

impl<LP: ToCss> ToCss for GenericLengthPercentageOrNormal<LP> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			GenericLengthPercentageOrNormal::LengthPercentage(value) => value.to_css(dest),
			GenericLengthPercentageOrNormal::Normal => dest.write_str("normal"),
		}
	}
}

#[derive(Clone, Debug)]
#[repr(C, u8)]
pub enum GenericLengthPercentageNumberOrNormal<Number, LengthPercent> {
	LengthPercentage(LengthPercent),
	Number(Number),
	Normal,
}

impl<N, LP> GenericLengthPercentageNumberOrNormal<N, LP> {
	pub fn parse_with<'i, 't, NP, LPP>(
		input: &mut Parser<'i, 't>,
		number_parser: NP,
		length_percentage_parser: LPP,
	) -> Result<Self, ParseError<'i>>
	where
		NP: Fn(&mut Parser<'i, 't>) -> Result<N, ParseError<'i>>,
		LPP: Fn(&mut Parser<'i, 't>) -> Result<LP, ParseError<'i>>,
	{
		input
			.try_parse(|input| {
				input.expect_ident_matching("normal")?;
				Ok(Self::Normal)
			})
			.or_else(|_err: ParseError<'i>| {
				let number = input.try_parse(|input| number_parser(input))?;
				Ok(Self::Number(number))
			})
			.or_else(|_err: ParseError<'i>| {
				let length_percent = length_percentage_parser(input)?;
				Ok(Self::LengthPercentage(length_percent))
			})
	}
}

impl<N: ToCss, LP: ToCss> ToCss for GenericLengthPercentageNumberOrNormal<N, LP> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			GenericLengthPercentageNumberOrNormal::LengthPercentage(value) => value.to_css(dest),
			GenericLengthPercentageNumberOrNormal::Number(value) => value.to_css(dest),
			GenericLengthPercentageNumberOrNormal::Normal => dest.write_str("normal"),
		}
	}
}

#[derive(Clone, Debug)]
pub enum GenericLengthOrNumber<N, L> {
	Number(N),
	Length(L),
}

impl<N, L> GenericLengthOrNumber<N, L> {
	pub fn parse_with<'i, 't, LP, NP>(
		input: &mut Parser<'i, 't>,
		number_parser: NP,
		length_parser: LP,
	) -> Result<Self, ParseError<'i>>
	where
		NP: FnOnce(&mut Parser<'i, 't>) -> Result<N, ParseError<'i>>,
		LP: FnOnce(&mut Parser<'i, 't>) -> Result<L, ParseError<'i>>,
	{
		input
			.try_parse(|input| {
				let number = number_parser(input)?;
				Ok(Self::Number(number))
			})
			.or_else(|_err: ParseError<'i>| {
				let length = length_parser(input)?;
				Ok(Self::Length(length))
			})
	}
}

impl<N: ToCss, L: ToCss> ToCss for GenericLengthOrNumber<N, L> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			GenericLengthOrNumber::Number(value) => value.to_css(dest),
			GenericLengthOrNumber::Length(value) => value.to_css(dest),
		}
	}
}

#[derive(Clone, Debug)]
pub struct Rect<T>(pub T, pub T, pub T, pub T)
where
	T: Clone;

impl<T: Clone> Rect<T> {
	pub fn parse_with<'i, 't, F>(input: &mut Parser<'i, 't>, item_parser: F) -> Result<Self, ParseError<'i>>
	where
		F: Fn(&mut Parser<'i, 't>) -> Result<T, ParseError<'i>>,
	{
		let first = item_parser(input)?;
		let mut state = input.state();
		let second = if let Ok(second) = item_parser(input) {
			second
		} else {
			input.reset(&state);
			return Ok(Self(first.clone(), first.clone(), first.clone(), first));
		};
		state = input.state();
		let third = if let Ok(third) = item_parser(input) {
			third
		} else {
			input.reset(&state);
			return Ok(Self(first.clone(), second.clone(), first, second));
		};
		state = input.state();
		let forth = if let Ok(forth) = item_parser(input) {
			forth
		} else {
			input.reset(&state);
			return Ok(Self(first, second.clone(), third, second));
		};
		Ok(Self(first, second, third, forth))
	}
}

impl<T: ToCss + Clone> ToCss for Rect<T> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		self.0.to_css(dest)?;
		dest.write_char(' ')?;
		self.1.to_css(dest)?;
		dest.write_char(' ')?;
		self.2.to_css(dest)?;
		dest.write_char(' ')?;
		self.3.to_css(dest)
	}
}

#[derive(Clone, Debug)]
pub enum GenericRectOrAuto<T: Clone> {
	Auto,
	Rect(Rect<T>),
}

impl<T: Clone> GenericRectOrAuto<T> {
	pub fn parse_with<'i, 't, F>(input: &mut Parser<'i, 't>, item_parser: F) -> Result<Self, ParseError<'i>>
	where
		F: Fn(&mut Parser<'i, 't>) -> Result<T, ParseError<'i>>,
	{
		input
			.try_parse(|input| {
				input.expect_ident_matching("auto")?;
				Ok(Self::Auto)
			})
			.or_else(|_err: ParseError<'i>| {
				let rect = Rect::parse_with(input, item_parser)?;
				Ok(Self::Rect(rect))
			})
	}
}

impl<T: Clone + ToCss> ToCss for GenericRectOrAuto<T> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			GenericRectOrAuto::Auto => dest.write_str("auto"),
			GenericRectOrAuto::Rect(rect) => rect.to_css(dest),
		}
	}
}
