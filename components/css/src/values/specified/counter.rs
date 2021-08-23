use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::{parse_repeated, ParseError};
use crate::properties::declaration::property_keywords_impl;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::generics::counter::{GenericCounter, GenericCounterOrNone};
use crate::values::image::Image;
use crate::values::number::Integer;
use crate::values::CustomIdent;

pub type CounterWithInteger = GenericCounterOrNone<GenericCounter<Integer>>;

impl CounterWithInteger {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        GenericCounterOrNone::parse_with(input, |input| {
            GenericCounter::parse_with(input, |input| Integer::parse(context, input))
        })
    }
}

#[derive(Clone)]
pub enum SymbolsType {
    Cyclic,
    Numeric,
    Alphabetic,
    Symbolic,
    Fixed,
}

property_keywords_impl! { SymbolsType,
    SymbolsType::Cyclic, "cyclic",
    SymbolsType::Numeric, "numeric",
    SymbolsType::Alphabetic, "alphabetic",
    SymbolsType::Symbolic, "symbolic",
    SymbolsType::Fixed, "fixed",
}

#[derive(Clone)]
pub enum StringOrImage {
    String(String),
    Image(Image),
}

impl StringOrImage {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let value = input.expect_string()?.to_string();
                Ok(StringOrImage::String(value))
            })
            .or_else(|_err: ParseError<'i>| {
                let image = Image::parse(context, input)?;
                Ok(StringOrImage::Image(image))
            })
    }
}

#[derive(Clone)]
pub struct Symbols {
    symbols_type: SymbolsType,
    idents: Vec<StringOrImage>,
}

impl Symbols {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input.expect_function_matching("symbols")?;
        input.parse_nested_block(|input| {
            let symbols_type = SymbolsType::parse(input)?;
            let idents =
                parse_repeated(input, &mut |input| StringOrImage::parse(context, input), 1)?;
            Ok(Symbols {
                symbols_type,
                idents,
            })
        })
    }
}

#[derive(Clone)]
pub enum CounterStyle {
    Name(CustomIdent),
    Symbols(Symbols),
}

impl CounterStyle {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let name = CustomIdent::parse(input)?;
                Ok(CounterStyle::Name(name))
            })
            .or_else(|_err: ParseError<'i>| {
                let symbols = Symbols::parse(context, input)?;
                Ok(CounterStyle::Symbols(symbols))
            })
    }
}

#[derive(Clone)]
pub struct InnerMostCounter {
    name: CustomIdent,
    style: Option<CounterStyle>,
}

impl InnerMostCounter {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let name = CustomIdent::parse_excluding(input, &["none"])?;
        let style = input
            .try_parse(|input| {
                input.expect_delim(',')?;
                CounterStyle::parse(context, input)
            })
            .ok();
        Ok(InnerMostCounter { name, style })
    }
}

#[derive(Clone)]
pub struct AllCounters {
    name: CustomIdent,
    string: String,
    style: Option<CounterStyle>,
}

impl AllCounters {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let name = CustomIdent::parse_excluding(input, &["none"])?;
        input.expect_delim(',')?;
        let str = input.expect_string()?.to_string();
        let style = input
            .try_parse(|input| {
                input.expect_delim(',')?;
                CounterStyle::parse(context, input)
            })
            .ok();
        Ok(AllCounters {
            name,
            string: str,
            style,
        })
    }
}

#[derive(Clone)]
pub enum Counter {
    Counter(InnerMostCounter),
    Counters(AllCounters),
}

impl Counter {
    /// https://drafts.csswg.org/css-lists-3/#typedef-counter
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let location = input.current_source_location();
        let name = input.expect_function()?;
        Ok(match_ignore_ascii_case! { name,
            "counter" => Counter::Counter(InnerMostCounter::parse(context, input)?),
            "counters" => Counter::Counters(AllCounters::parse(context, input)?),
            _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(name.clone())))
        })
    }
}
