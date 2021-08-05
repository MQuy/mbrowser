use std::fmt::Write;

use cssparser::{Parser, Token};

use super::CustomIdent;
use crate::css_writer::{CssWriter, ToCss};
use crate::parser::ParseError;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::{Integer, Number};

#[derive(Clone)]
#[repr(u8)]
pub enum TimingKeyword {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    StepStart,
    StepEnd,
}

#[derive(Clone)]
#[repr(u8)]
pub enum StepPosition {
    JumpStart,
    JumpEnd,
    JumpNone,
    JumpBoth,
    Start,
    End,
}

#[derive(Clone)]
#[repr(u8, C)]
pub enum TimingFunction {
    Keyword(TimingKeyword),
    CubicBezier {
        x1: Number,
        y1: Number,
        x2: Number,
        y2: Number,
    },
    Steps(Integer, StepPosition),
}

#[derive(Clone)]
pub enum KeyframesName {
    Ident(CustomIdent),
    QuotedString(String),
}

impl KeyframesName {
    pub fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let location = input.current_source_location();
        match input.next()? {
            Token::Ident(ref ident) => Ok(KeyframesName::Ident(CustomIdent::from_ident(
                location,
                ident,
                &["none"],
            )?)),
            Token::QuotedString(ref value) => Ok(KeyframesName::QuotedString(value.to_string())),
            t => Err(location.new_unexpected_token_error(t.clone())),
        }
    }
}

impl ToCss for KeyframesName {
    fn to_css<W>(&self, dest: &mut CssWriter<W>) -> core::fmt::Result
    where
        W: std::fmt::Write,
    {
        match self {
            KeyframesName::Ident(ident) => dest.write_str(&cssparser::ToCss::to_css_string(ident)),
            KeyframesName::QuotedString(value) => dest.write_str(value),
        }
    }
}
