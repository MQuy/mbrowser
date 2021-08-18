use cssparser::{match_ignore_ascii_case, Parser, _cssparser_internal_to_lowercase};

use crate::parser::{parse_repeated, ParseError};
use crate::properties::declaration::PropertyDeclaration;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub enum Quotes {
    Auto,
    None,
    Content(Vec<(String, String)>),
}

impl Quotes {
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Quotes, ParseError<'i>> {
        input.try_parse(|input| {
            let location = input.current_source_location();
            let ident = input.expect_ident()?;
            Ok(match_ignore_ascii_case! { ident,
                "auto" => Quotes::Auto,
                "none" => Quotes::None,
                _ => return Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
            })
        }).or_else(|_err: ParseError<'i>| {
            let pairs = parse_repeated(input, &mut |input| {
                let open_quote = input.expect_string()?.to_string();
                let close_quote = input.expect_string()?.to_string();
                Ok((open_quote, close_quote))
            }, 1)?;
            Ok(Quotes::Content(pairs))
        })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    Quotes::parse(context, input).map(PropertyDeclaration::Quotes)
}
