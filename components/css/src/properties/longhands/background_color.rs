use cssparser::Parser;

use crate::computed_values::StyleContext;
use crate::parser::ParseError;
use crate::properties::declaration::{PropertyDeclaration, WideKeywordDeclaration};
use crate::properties::longhand_id::LonghandId;
use crate::properties::property_id::CSSWideKeyword;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::computed;
use crate::values::specified::color::Color;

pub fn initial_value() -> Color {
	Color::Transparent
}

/// https://drafts.csswg.org/css-backgrounds/#background-color
pub fn cascade_property<'a>(declaration: Option<&PropertyDeclaration>, context: &'a mut StyleContext) {
	let computed_value = computed::from_non_inherited_property!(
		declaration,
		context.parent_style.get_background_color().clone(),
		initial_value().to_computed_value(context),
		LonghandId::BackgroundColor,
		PropertyDeclaration::BackgroundColor(value) => value.to_computed_value(context)
	);
	context.computed_values.set_background_color(computed_value);
}

pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	Color::parse(input).map(PropertyDeclaration::BackgroundColor)
}
