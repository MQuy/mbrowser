use cssparser::{Parser, ToCss};

use crate::css_writer::write_elements;
use crate::parser::{parse_repeated, ParseError};
use crate::values::CustomIdent;

#[derive(Clone, Debug)]
pub enum GenericCounterOrNone<Counter> {
	None,
	Counter(Vec<Counter>),
}

impl<C> GenericCounterOrNone<C> {
	pub fn parse_with<'i, 't, F>(input: &mut Parser<'i, 't>, item_parser: F) -> Result<Self, ParseError<'i>>
	where
		F: for<'a, 'b> Fn(&mut Parser<'a, 'b>) -> Result<C, ParseError<'a>>,
	{
		input
			.try_parse(|input| {
				input.expect_ident_matching("none")?;
				Ok(GenericCounterOrNone::None)
			})
			.or_else(|_err: ParseError<'i>| {
				let counters = parse_repeated(input, &mut |input| item_parser(input), 1)?;
				Ok(GenericCounterOrNone::Counter(counters))
			})
	}
}

impl<C: ToCss> ToCss for GenericCounterOrNone<C> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			GenericCounterOrNone::None => dest.write_str("none"),
			GenericCounterOrNone::Counter(counters) => {
				let values: Vec<String> = counters.iter().map(|v| v.to_css_string()).collect();
				dest.write_str(&values.join(" "))
			},
		}
	}
}

#[derive(Clone, Debug)]
pub struct GenericCounter<I> {
	name: CustomIdent,
	value: Option<I>,
}

impl<I> GenericCounter<I> {
	pub fn parse_with<'i, 't, F>(input: &mut Parser<'i, 't>, item_parser: F) -> Result<Self, ParseError<'i>>
	where
		F: Fn(&mut Parser<'i, 't>) -> Result<I, ParseError<'i>>,
	{
		let name = CustomIdent::parse(input)?;
		let value = input.try_parse(|input| item_parser(input)).ok();
		Ok(GenericCounter { name, value })
	}
}

impl<I: ToCss> ToCss for GenericCounter<I> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let name = Some(self.name.to_css_string());
		let value = self.value.as_ref().map(|v| v.to_css_string());
		write_elements(dest, &[name.as_deref(), value.as_deref()], ' ')
	}
}

#[derive(Clone, Debug)]
pub struct GenericReversedCounter<I> {
	name: CustomIdent,
	value: Option<I>,
	reversed: bool,
}

impl<I> GenericReversedCounter<I> {
	pub fn parse_with<'i, 't, F>(input: &mut Parser<'i, 't>, item_parser: F) -> Result<Self, ParseError<'i>>
	where
		F: Fn(&mut Parser<'i, 't>) -> Result<I, ParseError<'i>>,
	{
		input
			.try_parse(|input| {
				input.expect_function_matching("reversed")?;
				let name = input.parse_nested_block(|input| CustomIdent::parse(input))?;
				let value = input.try_parse(|input| item_parser(input)).ok();
				Ok(GenericReversedCounter {
					name,
					value,
					reversed: true,
				})
			})
			.or_else(|_err: ParseError<'i>| {
				let name = CustomIdent::parse(input)?;
				let value = input.try_parse(|input| item_parser(input)).ok();
				Ok(GenericReversedCounter {
					name,
					value,
					reversed: false,
				})
			})
	}
}

impl<I: ToCss> ToCss for GenericReversedCounter<I> {
	fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
	where
		W: std::fmt::Write,
	{
		let name = if self.reversed {
			Some(std::format!("{}({})", "reversed", self.name.to_css_string()))
		} else {
			Some(self.name.to_css_string())
		};
		let value = self.value.as_ref().map(|v| v.to_css_string());
		write_elements(dest, &[name.as_deref(), value.as_deref()], ' ')
	}
}
