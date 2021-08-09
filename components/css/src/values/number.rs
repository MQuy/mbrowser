use cssparser::Parser;

use super::CSSFloat;
use crate::parser::ParseError;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Number {
    value: CSSFloat,
}

impl Number {
    /// Parse a float.
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }

    pub fn get(&self) -> f32 {
        self.value
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
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        todo!()
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

pub type NonNegativeNumber = NonNegative<Number>;
