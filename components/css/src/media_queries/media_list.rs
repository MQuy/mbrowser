use cssparser::{Delimiter, Parser, Token};

use crate::stylesheets::stylesheet::ParserContext;

use super::media_query::MediaQuery;

/// A type that encapsulates a media query list.
#[derive(Clone)]
pub struct MediaList {
    /// The list of media queries.
    pub media_queries: Vec<MediaQuery>,
}

impl MediaList {
    /// Parse a media query list from CSS.
    ///
    /// Always returns a media query list. If any invalid media query is
    /// found, the media query list is only filled with the equivalent of
    /// "not all", see:
    ///
    /// <https://drafts.csswg.org/mediaqueries/#error-handling>
    pub fn parse(context: &ParserContext, input: &mut Parser) -> Self {
        if input.is_exhausted() {
            return Self::empty();
        }

        let mut media_queries = vec![];
        loop {
            let start_position = input.position();
            match input.parse_until_before(Delimiter::Comma, |i| MediaQuery::parse(context, i)) {
                Ok(mq) => {
                    media_queries.push(mq);
                },
                Err(err) => {
                    media_queries.push(MediaQuery::never_matching());
                    todo!()
                },
            }

            match input.next() {
                Ok(&Token::Comma) => {},
                Ok(_) => unreachable!(),
                Err(_) => break,
            }
        }

        MediaList { media_queries }
    }

    /// Create an empty MediaList.
    pub fn empty() -> Self {
        MediaList {
            media_queries: vec![],
        }
    }

    /// Whether this `MediaList` contains no media queries.
    pub fn is_empty(&self) -> bool {
        self.media_queries.is_empty()
    }
}
