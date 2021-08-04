use crate::stylesheets::rule_parser::StyleParseErrorKind;

pub type ParseError<'i> = cssparser::ParseError<'i, StyleParseErrorKind<'i>>;
