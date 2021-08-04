use cssparser::{Parser, SourceLocation};

use super::rule_parser::VendorPrefix;
use super::stylesheet::ParserContext;
use crate::properties::declaration_block::PropertyDeclarationBlock;
use crate::properties::longhands::animation_name::KeyframesName;

/// A [`@keyframes`][keyframes] rule.
///
/// [keyframes]: https://drafts.csswg.org/css-animations/#keyframes
#[derive(Clone)]
pub struct KeyframesRule {
    /// The name of the current animation.
    pub name: KeyframesName,
    /// The keyframes specified for this CSS rule.
    pub keyframes: Vec<Keyframe>,
    /// Vendor prefix type the @keyframes has.
    pub vendor_prefix: Option<VendorPrefix>,
    /// The line and column of the rule's source code.
    pub source_location: SourceLocation,
}
/// A keyframe.
#[derive(Clone)]
pub struct Keyframe {
    /// The selector this keyframe was specified from.
    pub selector: Vec<f32>,

    /// The declaration block that was declared inside this keyframe.
    ///
    /// Note that `!important` rules in keyframes don't apply, but we keep this
    pub block: PropertyDeclarationBlock,

    /// The line and column of the rule's source code.
    pub source_location: SourceLocation,
}

/// Parses a keyframe list from CSS input.
pub fn parse_keyframe_list(context: &ParserContext, input: &mut Parser) -> Vec<Keyframe> {
    todo!()
}
