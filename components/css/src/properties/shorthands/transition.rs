use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::properties::longhands::transition_delay::TransitionDelay;
use crate::properties::longhands::transition_duration::TransitionDuration;
use crate::properties::longhands::transition_property::TransitionProperty;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::animation::TimingFunction;

pub struct Longhands {
    pub transition_property: TransitionProperty,
    pub transition_duration: TransitionDuration,
    pub transition_timing_function: TimingFunction,
    pub transition_delay: TransitionDelay,
}

pub fn parse_value<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<Longhands, ParseError<'i>> {
    todo!()
}

/// Parse the given shorthand and fill the result into the
/// `declarations` vector.
pub fn parse_into<'i, 't>(
    declarations: &mut SourcePropertyDeclaration,
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<(), ParseError<'i>> {
    input
        .parse_entirely(|input| parse_value(context, input))
        .map(|longhands| {
            declarations.push(PropertyDeclaration::TransitionProperty(
                longhands.transition_property,
            ));
            declarations.push(PropertyDeclaration::TransitionDuration(
                longhands.transition_duration,
            ));
            declarations.push(PropertyDeclaration::TransitionTimingFunction(
                longhands.transition_timing_function,
            ));
            declarations.push(PropertyDeclaration::TransitionDelay(
                longhands.transition_delay,
            ));
        })
}
