use cssparser::Parser;

use crate::computed_values::StyleContext;
use crate::parser::ParseError;
use crate::properties::declaration::{PropertyDeclaration, WideKeywordDeclaration};
use crate::properties::longhand_id::LonghandId;
use crate::properties::property_id::CSSWideKeyword;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::computed;
use crate::values::specified::color::{Color, RGBA};

pub fn initial_value() -> RGBA {
	RGBA::transparent()
}

pub fn cascade_property<'a>(declaration: Option<&PropertyDeclaration>, context: &'a mut StyleContext) {
	let computed_value = computed::from_non_inherited_property!(
		declaration,
		context.parent_style.get_border_bottom_color().clone(),
		initial_value(),
		LonghandId::BorderBottomColor,
		PropertyDeclaration::BorderBottomColor(value) => value.to_computed_value(context)
	);
	context.computed_values.set_border_bottom_color(computed_value);
}

/// https://drafts.csswg.org/css-backgrounds/#propdef-border-bottom-color
pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	Color::parse(input).map(PropertyDeclaration::BorderBottomColor)
}
