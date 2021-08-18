use cssparser::{match_ignore_ascii_case, Parser, _cssparser_internal_to_lowercase};

use crate::parser::{parse_repeated, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::image::Image;
use crate::values::specified::counter::Counter;
use crate::values::specified::leader::Leader;
use crate::values::specified::quote::Quote;
use crate::values::specified::target::Target;

#[derive(Clone)]
pub enum ContentList {
    String(String),
    Contents,
    Image(Image),
    Counter(Counter),
    Quote(Quote),
    Target(Target),
    Leader(Leader),
}

impl ContentList {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                input.expect_ident_matching("contents")?;
                Ok(ContentList::Contents)
            })
            .or_else(|_err: ParseError<'i>| {
                input.try_parse(|input| {
                    let value = input.expect_string()?.to_string();
                    Ok(ContentList::String(value))
                })
            })
            .or_else(|_err: ParseError<'i>| {
                input.try_parse(|input| {
                    let image = Image::parse(context, input)?;
                    Ok(ContentList::Image(image))
                })
            })
            .or_else(|_err: ParseError<'i>| {
                input.try_parse(|input| {
                    let counter = Counter::parse(context, input)?;
                    Ok(ContentList::Counter(counter))
                })
            })
            .or_else(|_err: ParseError<'i>| {
                input.try_parse(|input| {
                    let quote = Quote::parse(input)?;
                    Ok(ContentList::Quote(quote))
                })
            })
            .or_else(|_err: ParseError<'i>| {
                input.try_parse(|input| {
                    let target = Target::parse(context, input)?;
                    Ok(ContentList::Target(target))
                })
            })
            .or_else(|_err: ParseError<'i>| {
                input.try_parse(|input| {
                    let leader = Leader::parse(context, input)?;
                    Ok(ContentList::Leader(leader))
                })
            })
    }
}

#[derive(Clone)]
pub enum ContentReplacementOrList {
    Replacement(Image),
    List(ContentList),
}

impl ContentReplacementOrList {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let image = Image::parse(context, input)?;
                Ok(ContentReplacementOrList::Replacement(image))
            })
            .or_else(|_err: ParseError<'i>| {
                let value = ContentList::parse(context, input)?;
                Ok(ContentReplacementOrList::List(value))
            })
    }
}

#[derive(Clone)]
pub enum CounterOrString {
    Counter(Counter),
    String(String),
}

impl CounterOrString {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let value = Counter::parse(context, input)?;
                Ok(CounterOrString::Counter(value))
            })
            .or_else(|_err: ParseError<'i>| {
                let value = input.expect_string()?.to_string();
                Ok(CounterOrString::String(value))
            })
    }
}

#[derive(Clone)]
pub struct ContentData {
    content: ContentReplacementOrList,
    alt: Vec<CounterOrString>,
}

impl ContentData {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let content = ContentReplacementOrList::parse(context, input)?;
        let alt = input
            .try_parse(|input| -> Result<Vec<CounterOrString>, ParseError<'i>> {
                input.expect_delim('/')?;
                let value = parse_repeated(
                    input,
                    &mut |input| CounterOrString::parse(context, input),
                    1,
                )?;
                Ok(value)
            })
            .map_or(vec![], |alt| alt);
        Ok(ContentData { content, alt })
    }
}

#[derive(Clone)]
pub enum Content {
    Normal,
    None,
    Data(ContentData),
}

impl Content {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input.try_parse(|input| {
            let location = input.current_source_location();
            let ident = input.expect_ident()?;
            Ok(match_ignore_ascii_case! { ident,
                "normal" => Content::Normal,
                "none" => Content::None,
                _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
            })
        }).or_else(|_err: ParseError<'i>| {
            let value = ContentData::parse(context, input)?;
            Ok(Content::Data(value))
        })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    Content::parse(context, input).map(PropertyDeclaration::Content)
}
