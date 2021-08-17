use cssparser::Parser;

use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

pub type ParseError<'i> = cssparser::ParseError<'i, StyleParseErrorKind<'i>>;

pub fn parse_item_if_missing<'i, 't, T, F>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
    item: &mut Option<T>,
    item_parser: F,
) -> Result<(), ParseError<'i>>
where
    T: PartialEq,
    F: Fn(&ParserContext, &mut Parser<'i, 't>) -> Result<T, ParseError<'i>>,
{
    if item.is_none() {
        input.try_parse(|input| -> Result<(), ParseError<'i>> {
            *item = Some(item_parser(context, input)?);
            Ok(())
        })
    } else {
        Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
    }
}

pub fn parse_in_any_order<'i, 't, T>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
    item_parsers: &mut [&mut dyn FnMut(
        &ParserContext,
        &mut Parser<'i, 't>,
    ) -> Result<T, ParseError<'i>>],
) {
    loop {
        let state = input.state();

        if item_parsers
            .iter_mut()
            .map(|item_parser| item_parser(context, input))
            .all(|ret| ret.is_err())
        {
            input.reset(&state);
            break;
        }
    }
}

pub fn parse_repeated<'i, 't, F, T>(
    input: &mut Parser<'i, 't>,
    item_parser: &mut F,
    minimum_times: usize,
) -> Result<Vec<T>, ParseError<'i>>
where
    F: FnMut(&mut Parser<'i, 't>) -> Result<T, ParseError<'i>>,
{
    let mut items = Vec::with_capacity(minimum_times);
    let mut counter = 0;
    loop {
        let state = input.state();
        let ret = item_parser(input);

        if let Ok(item) = ret {
            items.push(item);
        } else {
            input.reset(&state);

            if counter < minimum_times {
                return ret.map(|_| vec![]);
            } else {
                break;
            }
        }
        counter += 1;
    }
    Ok(items)
}
