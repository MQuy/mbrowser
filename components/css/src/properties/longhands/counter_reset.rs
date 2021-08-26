use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::generics::counter::{GenericCounterOrNone, GenericReversedCounter};
use crate::values::number::Integer;

/// https://drafts.csswg.org/css-lists/#counter-reset
pub type CounterReset = GenericCounterOrNone<GenericReversedCounter<Integer>>;

impl CounterReset {
	pub fn parse<'i, 't>(
		context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		GenericCounterOrNone::parse_with(input, |input| {
			GenericReversedCounter::parse_with(input, |input| Integer::parse(context, input))
		})
	}
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	CounterReset::parse(context, input).map(PropertyDeclaration::CounterReset)
}
