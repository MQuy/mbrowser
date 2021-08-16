use cssparser::Parser;

use crate::stylesheets::rule_parser::StyleParseErrorKind;

pub type ParseError<'i> = cssparser::ParseError<'i, StyleParseErrorKind<'i>>;

pub fn parse_item_if_missing<'i, 't, T, F>(
    input: &mut Parser<'i, 't>,
    item: &mut Option<T>,
    item_parser: F,
) -> Result<(), ParseError<'i>>
where
    T: PartialEq,
    F: Fn(&mut Parser<'i, 't>) -> Result<T, ParseError<'i>>,
{
    if item.is_none() {
        input.try_parse(|input| -> Result<(), ParseError<'i>> {
            *item = Some(item_parser(input)?);
            Ok(())
        })
    } else {
        Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError))
    }
}

pub fn parse_when<'i, 't, F>(input: &mut Parser<'i, 't>, item_parser: &mut F)
where
    F: FnMut(&mut Parser<'i, 't>) -> Vec<Result<(), ParseError<'i>>>,
{
    loop {
        let state = input.state();
        let ret = item_parser(input);

        if ret.iter().all(|ret| ret.is_err()) {
            input.reset(&state);
            break;
        }
    }
}
