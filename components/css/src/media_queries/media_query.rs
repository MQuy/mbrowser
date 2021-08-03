use cssparser::Parser;

use crate::parser::ParseError;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::CustomIdent;

use super::media_condition::MediaCondition;

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
pub struct MediaType(pub CustomIdent);

/// <http://dev.w3.org/csswg/mediaqueries-3/#media0>
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MediaQueryType {
    /// A media type that matches every device.
    All,
    /// A specific media type.
    Concrete(MediaType),
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
        todo!()
    }
}
