use cssparser::Parser;

use crate::computed_values::StyleContext;
use crate::parser::ParseError;
use crate::properties::declaration::{PropertyDeclaration, WideKeywordDeclaration};
use crate::properties::longhand_id::LonghandId;
use crate::properties::property_id::CSSWideKeyword;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::computed;
use crate::values::specified::color::{Color, SystemColor};

pub fn initial_value() -> Color {
	Color::System(SystemColor::CanvasText)
}

/// https://drafts.csswg.org/css-color/#color-syntax
pub fn cascade_property<'a>(declaration: Option<&PropertyDeclaration>, context: &'a mut StyleContext) {
	let computed_value = computed::from_inherited_property!(
		declaration,
		context.parent_style.get_color().clone(),
		initial_value().to_computed_value(context),
		LonghandId::Color,
		PropertyDeclaration::Color(value) => value.to_computed_value(context)
	);
	context.computed_values.set_color(computed_value);
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	let color = Color::parse(context, input)?;
	Ok(match color {
		Color::CurrentColor => PropertyDeclaration::CSSWideKeyword(WideKeywordDeclaration {
			id: LonghandId::Color,
			keyword: CSSWideKeyword::Inherit,
		}),
		value => PropertyDeclaration::Color(value),
	})
}
