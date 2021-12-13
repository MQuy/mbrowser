use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::shortcut_for_four_values;
use crate::values::specified::layout::LineStyle;

pub struct Longhands {
	pub border_top_style: LineStyle,
	pub border_right_style: LineStyle,
	pub border_bottom_style: LineStyle,
	pub border_left_style: LineStyle,
}

shortcut_for_four_values!(
	Longhands,
	border_top_style,
	border_right_style,
	border_bottom_style,
	border_left_style,
	LineStyle
);

/// Parse the given shorthand and fill the result into the
/// `declarations` vector.
pub fn parse_into<'i, 't>(
	declarations: &mut SourcePropertyDeclaration,
	_context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i>> {
	input
		.parse_entirely(|input| Longhands::parse_values(input))
		.map(|longhands| {
			declarations.push(PropertyDeclaration::BorderTopStyle(longhands.border_top_style));
			declarations.push(PropertyDeclaration::BorderRightStyle(longhands.border_right_style));
			declarations.push(PropertyDeclaration::BorderBottomStyle(longhands.border_bottom_style));
			declarations.push(PropertyDeclaration::BorderLeftStyle(longhands.border_left_style));
		})
}
