use std::cmp::Ordering;
use std::fmt::Display;

use cssparser::{Parser, ToCss};

use super::CSSFloat;
use crate::parser::ParseError;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

/// https://www.w3.org/TR/css-values-4/#numbers
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Number {
    value: CSSFloat,
}

impl Number {
    pub fn new(value: CSSFloat) -> Self {
        Number { value }
    }

    /// Parse a float.
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let value = input.expect_number()?;
        Ok(Number { value })
    }

    pub fn parse_non_negative<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let value = Number::parse(context, input)?;
        if value < 0 {
            return Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError));
        }
        Ok(value)
    }

    pub fn get(&self) -> f32 {
        self.value
    }
}

impl PartialEq<i32> for Number {
    fn eq(&self, other: &i32) -> bool {
        self.get() as i32 == *other
    }
}

impl PartialOrd<i32> for Number {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        let value = self.get() as i32;
        if value > *other {
            Some(Ordering::Greater)
        } else if value < *other {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.value))
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct NonNegative<T>(pub T);

#[derive(Clone, PartialEq, PartialOrd)]
pub struct GreaterThanOrEqualToOne<T>(pub T);

pub type PositiveInteger = GreaterThanOrEqualToOne<Integer>;

#[derive(Clone)]
pub struct Integer(i32);

impl Integer {
    /// Parse a non-negative integer.
    pub fn parse_non_negative<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let value = input.expect_integer()?;
        if value < 0 {
            return Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError));
        }
        Ok(Integer(value))
    }

    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let value = input.expect_integer()?;
        Ok(Integer(value))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

pub type NonNegativeNumber = NonNegative<Number>;

impl NonNegativeNumber {
    pub fn new(val: CSSFloat) -> Self {
        NonNegative::<Number>(Number::new(val.max(0.)))
    }

    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let value = Number::parse_non_negative(context, input)?;
        Ok(NonNegative::<Number>(value))
    }
}

impl ToCss for NonNegativeNumber {
    fn to_css<W>(&self, dest: &mut W) -> std::fmt::Result
    where
        W: std::fmt::Write,
    {
        dest.write_fmt(format_args!("{}", self.0))
    }
}

/// Generic for Number/Auto
#[derive(Clone)]
pub enum GenericNumberOrAuto<Number> {
    Number(Number),
    Auto,
}

impl<L> GenericNumberOrAuto<L> {
    pub fn parse_with<'i, 't, LP>(
        input: &mut Parser<'i, 't>,
        number_parser: LP,
    ) -> Result<Self, ParseError<'i>>
    where
        LP: FnOnce(&mut Parser<'i, 't>) -> Result<L, ParseError<'i>>,
    {
        input
            .try_parse(|input| {
                input.expect_ident_matching("auto")?;
                Ok(Self::Auto)
            })
            .or_else(|_err: ParseError<'i>| {
                let length = number_parser(input)?;
                Ok(Self::Number(length))
            })
    }
}

pub type IntegerAuto = GenericNumberOrAuto<Integer>;

impl IntegerAuto {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        Self::parse_with(input, |input| Integer::parse(context, input))
    }
}
