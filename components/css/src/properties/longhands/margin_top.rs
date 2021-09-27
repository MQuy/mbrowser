use cssparser::Parser;

use crate::computed_values::StyleContext;
use crate::parser::ParseError;
use crate::properties::declaration::{PropertyDeclaration, WideKeywordDeclaration};
use crate::properties::longhand_id::LonghandId;
use crate::properties::property_id::CSSWideKeyword;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::LengthPercentageOrAuto;

pub fn initial_value() -> LengthPercentageOrAuto {
	LengthPercentageOrAuto::zero()
}

pub fn cascade_property<'a>(
	declaration: Option<&PropertyDeclaration>,
	context: &'a mut StyleContext,
) {
	let initial_value = initial_value();
	let specified_value = match declaration {
		Some(declaration) => match declaration {
			PropertyDeclaration::MarginTop(value) => value,
			PropertyDeclaration::CSSWideKeyword(WideKeywordDeclaration { id, keyword })
				if *id == LonghandId::MarginTop =>
			{
				match keyword {
					CSSWideKeyword::Initial | CSSWideKeyword::Unset => &initial_value,
					CSSWideKeyword::Inherit => context.parent_style.get_margin_top(),
					CSSWideKeyword::Revert => unreachable!(),
				}
			},
			_ => unreachable!(),
		},
		None => &initial_value,
	};
	context
		.computed_values
		.set_margin_top(specified_value.clone());
}

/// https://drafts.csswg.org/css-box-4/#propdef-margin-top
pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	LengthPercentageOrAuto::parse(context, input).map(PropertyDeclaration::MarginTop)
}
