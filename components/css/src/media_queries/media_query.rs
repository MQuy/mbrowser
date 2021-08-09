use cssparser::Parser;

use super::media_condition::MediaCondition;
use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::Ident;

/// <https://drafts.csswg.org/mediaqueries/#mq-prefix>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Qualifier {
    /// Hide a media query from legacy UAs:
    /// <https://drafts.csswg.org/mediaqueries/#mq-only>
    Only,
    /// Negate a media query:
    /// <https://drafts.csswg.org/mediaqueries/#mq-not>
    Not,
}

/// <https://drafts.csswg.org/mediaqueries/#media-types>
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaType(pub Ident);

/// <http://dev.w3.org/csswg/mediaqueries-3/#media0>
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MediaQueryType {
    /// A media type that matches every device.
    All,
    /// A specific media type.
    Concrete(MediaType),
}

impl MediaQueryType {
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let ident = input.expect_ident()?.to_string();
        if ident == "all" {
            Ok(MediaQueryType::All)
        } else {
            Ok(MediaQueryType::Concrete(MediaType(Ident(ident))))
        }
    }
}

/// A [media query][mq].
///
/// [mq]: https://drafts.csswg.org/mediaqueries/
#[derive(Clone, Debug, PartialEq)]
pub struct MediaQuery {
    /// The qualifier for this query.
    pub qualifier: Option<Qualifier>,
    /// The media type for this query, that can be known, unknown, or "all".
    pub media_type: MediaQueryType,
    /// The condition that this media query contains. This cannot have `or`
    /// in the first level.
    pub condition: Option<MediaCondition>,
}

impl MediaQuery {
    /// Return a media query that never matches, used for when we fail to parse
    /// a given media query.
    pub fn never_matching() -> Self {
        Self {
            qualifier: Some(Qualifier::Not),
            media_type: MediaQueryType::All,
            condition: None,
        }
    }

    /// Parse a media query given css input.
    ///
    /// Returns an error if any of the expressions is unknown.
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| MediaCondition::parse(context, input))
            .map(|media_condition| MediaQuery {
                qualifier: None,
                media_type: MediaQueryType::All,
                condition: Some(media_condition),
            })
            .or_else(|_err| {
                let qualifier = match input.expect_ident() {
                    Ok(ident) => match ident.to_string() {
                        value if value == "not" => Some(Qualifier::Not),
                        value if value == "only" => Some(Qualifier::Only),
                        _ => {
                            return Err(input
                                .new_custom_error(StyleParseErrorKind::MediaQueryExpectedToken));
                        },
                    },
                    _ => None,
                };
                let media_type = MediaQueryType::parse(context, input)?;
                let condition = input
                    .try_parse(|input| MediaCondition::parse(context, input))
                    .ok();
                Ok(MediaQuery {
                    qualifier,
                    media_type,
                    condition,
                })
            })
    }
}
