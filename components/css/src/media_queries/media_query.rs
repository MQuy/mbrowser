use core::fmt;
use std::fmt::Write;

use cssparser::{Parser, Token};

use super::media_condition::MediaCondition;
use crate::css_writer::ToCss;
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

impl ToCss for Qualifier {
	fn to_css<W>(&self, dest: &mut crate::css_writer::CssWriter<W>) -> fmt::Result
	where
		W: Write,
	{
		match self {
			Qualifier::Only => dest.write_str("only"),
			Qualifier::Not => dest.write_str("not"),
		}
	}
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
	pub fn parse<'i, 't>(_context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
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
	pub fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		input
			.try_parse(|input| MediaCondition::parse(context, input))
			.map(|media_condition| MediaQuery {
				qualifier: None,
				media_type: MediaQueryType::All,
				condition: Some(media_condition),
			})
			.or_else(|_err| {
				let qualifier = input
					.try_parse(|input| -> Result<Qualifier, ParseError<'i>> {
						match input.next()? {
							Token::Ident(ident) => match ident.to_string() {
								value if value == "not" => Ok(Qualifier::Not),
								value if value == "only" => Ok(Qualifier::Only),
								_ => return Err(input.new_custom_error(StyleParseErrorKind::MediaQueryExpectedToken)),
							},
							_ => return Err(input.new_custom_error(StyleParseErrorKind::MediaQueryExpectedToken)),
						}
					})
					.ok();
				let media_type = MediaQueryType::parse(context, input)?;
				let condition = input
					.try_parse(|input| {
						input.expect_ident_matching("and")?;
						MediaCondition::parse_without_or(context, input)
					})
					.ok();
				Ok(MediaQuery {
					qualifier,
					media_type,
					condition,
				})
			})
	}
}

impl ToCss for MediaQuery {
	fn to_css<W>(&self, dest: &mut crate::css_writer::CssWriter<W>) -> fmt::Result
	where
		W: std::fmt::Write,
	{
		if let Some(qualifier) = self.qualifier {
			qualifier.to_css(dest)?;
			dest.write_char(' ')?;
		}

		match &self.media_type {
			MediaQueryType::All => {
				if self.qualifier.is_some() || self.condition.is_none() {
					dest.write_str("all")?;
				}
			},
			MediaQueryType::Concrete(MediaType(ident)) => dest.write_str(&ident.0)?,
		};

		if self.media_type != MediaQueryType::All || self.qualifier.is_some() {
			dest.write_str(" and ")?;
		}

		self.condition.to_css(dest)
	}
}
