use cssparser::Parser;

use crate::computed_values::StyleContext;
use crate::parser::ParseError;
use crate::properties::declaration::{PropertyDeclaration, WideKeywordDeclaration};
use crate::properties::longhand_id::LonghandId;
use crate::properties::property_id::CSSWideKeyword;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::computed;
use crate::values::specified::length::NonNegativeLengthPercentage;

pub fn initial_value() -> NonNegativeLengthPercentage {
	NonNegativeLengthPercentage::zero()
}

pub fn cascade_property<'a>(declaration: Option<&PropertyDeclaration>, context: &'a mut StyleContext) {
	let computed_value = computed::from_non_inherited_property!(
		declaration,
		context.parent_style.get_padding_right().clone(),
		initial_value().to_computed_value(context),
		LonghandId::PaddingRight,
		PropertyDeclaration::PaddingRight(value) => value.to_computed_value(context)
	);
	context.computed_values.set_padding_right(computed_value);
}

/// https://drafts.csswg.org/css-box-4/#propdef-padding-right
pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	NonNegativeLengthPercentage::parse(context, input).map(PropertyDeclaration::PaddingRight)
}
