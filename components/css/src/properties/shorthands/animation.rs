use cssparser::Parser;

use crate::parser::ParseError;
use crate::properties::declaration::PropertyDeclaration;
use crate::properties::declaration_block::SourcePropertyDeclaration;
use crate::properties::longhands::animation_delay::AnimationDelay;
use crate::properties::longhands::animation_direction::AnimationDirection;
use crate::properties::longhands::animation_duration::AnimationDuration;
use crate::properties::longhands::animation_fill_mode::AnimationFillMode;
use crate::properties::longhands::animation_iteration_count::AnimationIterationCount;
use crate::properties::longhands::animation_name::AnimationName;
use crate::properties::longhands::animation_play_state::AnimationPlayState;
use crate::properties::longhands::animation_timing_function::AnimationTimingFunction;
use crate::stylesheets::stylesheet::ParserContext;

pub struct Longhands {
    pub animation_name: AnimationName,
    pub animation_duration: AnimationDuration,
    pub animation_timing_function: AnimationTimingFunction,
    pub animation_delay: AnimationDelay,
    pub animation_iteration_count: AnimationIterationCount,
    pub animation_direction: AnimationDirection,
    pub animation_fill_mode: AnimationFillMode,
    pub animation_play_state: AnimationPlayState,
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
            declarations.push(PropertyDeclaration::AnimationName(longhands.animation_name));
            declarations.push(PropertyDeclaration::AnimationDuration(
                longhands.animation_duration,
            ));
            declarations.push(PropertyDeclaration::AnimationTimingFunction(
                longhands.animation_timing_function,
            ));
            declarations.push(PropertyDeclaration::AnimationDelay(
                longhands.animation_delay,
            ));
            declarations.push(PropertyDeclaration::AnimationIterationCount(
                longhands.animation_iteration_count,
            ));
            declarations.push(PropertyDeclaration::AnimationDirection(
                longhands.animation_direction,
            ));
            declarations.push(PropertyDeclaration::AnimationFillMode(
                longhands.animation_fill_mode,
            ));
            declarations.push(PropertyDeclaration::AnimationPlayState(
                longhands.animation_play_state,
            ));
        })
}
