use common::url::BrowserUrl;
use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::parser::{parse_repeated, ParseError};
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;
use crate::values::number::Number;

#[derive(Clone)]
#[repr(u8)]
pub enum CursorKind {
    None,
    Default,
    Pointer,
    ContextMenu,
    Help,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    VerticalText,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
    AllScroll,
    ZoomIn,
    ZoomOut,
    Auto,
}

property_keywords_impl! { CursorKind,
    CursorKind::None, "none",
    CursorKind::Default, "default",
    CursorKind::Pointer, "pointer",
    CursorKind::ContextMenu, "context-menu",
    CursorKind::Help, "help",
    CursorKind::Progress, "progress",
    CursorKind::Wait, "wait",
    CursorKind::Cell, "cell",
    CursorKind::Crosshair, "crosshair",
    CursorKind::Text, "text",
    CursorKind::VerticalText, "vertical-text",
    CursorKind::Alias, "alias",
    CursorKind::Copy, "copy",
    CursorKind::Move, "move",
    CursorKind::NoDrop, "no-drop",
    CursorKind::NotAllowed, "not-allowed",
    CursorKind::Grab, "grab",
    CursorKind::Grabbing, "grabbing",
    CursorKind::EResize, "e-resize",
    CursorKind::NResize, "n-resize",
    CursorKind::NeResize, "ne-resize",
    CursorKind::NwResize, "nw-resize",
    CursorKind::SResize, "s-resize",
    CursorKind::SeResize, "se-resize",
    CursorKind::SwResize, "sw-resize",
    CursorKind::WResize, "w-resize",
    CursorKind::EwResize, "ew-resize",
    CursorKind::NsResize, "ns-resize",
    CursorKind::NeswResize, "nesw-resize",
    CursorKind::NwseResize, "nwse-resize",
    CursorKind::ColResize, "col-resize",
    CursorKind::RowResize, "row-resize",
    CursorKind::AllScroll, "all-scroll",
    CursorKind::ZoomIn, "zoom-in",
    CursorKind::ZoomOut, "zoom-out",
    CursorKind::Auto, "auto",
}

#[derive(Clone)]
#[repr(C)]
pub struct CursorImage {
    pub url: BrowserUrl,
    pub x: Option<Number>,
    pub y: Option<Number>,
}

impl CursorImage {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let value = input.expect_ident_or_string()?;
        let url = BrowserUrl::parse(value)
            .map_err(|_err| input.new_custom_error(StyleParseErrorKind::UnspecifiedError))?;
        let (x, y) = input
            .try_parse(|input| -> Result<(Number, Number), ParseError<'i>> {
                let x = Number::parse(context, input)?;
                let y = Number::parse(context, input)?;
                Ok((x, y))
            })
            .map_or((None, None), |(x, y)| (Some(x), Some(y)));
        Ok(CursorImage { url, x, y })
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct Cursor {
    pub images: Vec<CursorImage>,
    pub keyword: CursorKind,
}

impl Cursor {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Cursor, ParseError<'i>> {
        let images = parse_repeated(input, &mut |input| CursorImage::parse(context, input), 0)?;
        let keyword = CursorKind::parse(input)?;
        Ok(Cursor { images, keyword })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    Cursor::parse(context, input).map(PropertyDeclaration::Cursor)
}
