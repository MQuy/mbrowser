use cssparser::{match_ignore_ascii_case, Parser, Token, _cssparser_internal_to_lowercase};

use crate::parser::ParseError;
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::length::Pair;

#[derive(Clone)]
pub enum BorderImageRepeatKeyword {
    Stretch,
    Repeat,
    Round,
    Space,
}

property_keywords_impl! { BorderImageRepeatKeyword,
    BorderImageRepeatKeyword::Stretch, "stretch",
    BorderImageRepeatKeyword::Repeat, "repeat",
    BorderImageRepeatKeyword::Round, "round",
    BorderImageRepeatKeyword::Space, "space",
}

pub type BorderImageRepeat = Pair<BorderImageRepeatKeyword>;

impl BorderImageRepeat {
    /// https://drafts.csswg.org/css-backgrounds/#the-border-image-repeat
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        BorderImageRepeat::parse_pair(input, |input| BorderImageRepeatKeyword::parse(input))
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    BorderImageRepeat::parse(context, input).map(PropertyDeclaration::BorderImageRepeat)
}
