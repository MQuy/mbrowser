use cssparser::Parser;

use crate::computed_values::StyleContext;
use crate::parser::ParseError;
use crate::properties::declaration::{PropertyDeclaration, WideKeywordDeclaration};
use crate::properties::longhand_id::LonghandId;
use crate::properties::property_id::CSSWideKeyword;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::computed;
use crate::values::specified::layout::LineStyle;

pub fn initial_value() -> LineStyle {
	LineStyle::None
}

pub fn cascade_property<'a>(declaration: Option<&PropertyDeclaration>, context: &'a mut StyleContext) {
	let computed_value = computed::from_non_inherited_property!(
		declaration,
		context.parent_style.get_border_top_style().clone(),
		initial_value(),
		LonghandId::BorderTopStyle,
		PropertyDeclaration::BorderTopStyle(value) => value.clone()
	);
	context.computed_values.set_border_top_style(computed_value);
}

/// https://drafts.csswg.org/css-backgrounds-3/#propdef-border-top-style
pub fn parse_declared<'i, 't>(
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	LineStyle::parse(input).map(PropertyDeclaration::BorderTopStyle)
}
