use cssparser::{match_ignore_ascii_case, Parser, Token, _cssparser_internal_to_lowercase};

use crate::parser::{parse_in_any_order, parse_item_if_missing, ParseError};
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone, PartialEq)]
pub enum DisplayOutside {
    Block,
    Inline,
    RunIn,
}

property_keywords_impl! { DisplayOutside,
    DisplayOutside::Block, "block",
    DisplayOutside::Inline, "inline",
    DisplayOutside::RunIn, "run-in",
}

#[derive(Clone, PartialEq)]
pub enum DisplayInside {
    Flow,
    FlowRoot,
    Table,
    Flex,
    Grid,
    Ruby,
}

#[derive(Clone)]
pub struct DisplayBasic {
    outside: Option<DisplayOutside>,
    inside: Option<DisplayInside>,
}

impl DisplayBasic {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let mut outside = None;
        let mut inside = None;
        parse_in_any_order(
            context,
            input,
            &mut [
                &mut |context, input| {
                    parse_item_if_missing(context, input, &mut outside, |_, input| {
                        DisplayOutside::parse(input)
                    })
                },
                &mut |context, input| {
                    parse_item_if_missing(context, input, &mut inside, |_, input| {
                        DisplayInside::parse(input)
                    })
                },
            ],
        );
        Ok(DisplayBasic { outside, inside })
    }
}

property_keywords_impl! { DisplayInside,
    DisplayInside::Flow, "flow",
    DisplayInside::FlowRoot, "flow-root",
    DisplayInside::Table, "table",
    DisplayInside::Flex, "flex",
    DisplayInside::Grid, "grid",
    DisplayInside::Ruby, "ruby",
}

#[derive(Clone)]
pub struct DisplayListItem {
    outside: Option<DisplayOutside>,
    inside: Option<DisplayInside>, // only allow flow, flow-root
}

impl DisplayListItem {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        let mut outside = None;
        let mut inside = None;
        let mut item = None;
        parse_in_any_order(
            context,
            input,
            &mut [
                &mut |context, input| {
                    parse_item_if_missing(context, input, &mut outside, |_, input| {
                        DisplayOutside::parse(input)
                    })
                },
                &mut |context, input| {
                    parse_item_if_missing(context, input, &mut inside, |_, input| {
                        let location = input.current_source_location();
                        let ident = input.expect_ident()?;
                        match_ignore_ascii_case! { ident,
                            "flow" => Ok(DisplayInside::Flow),
                            "flow-root" => Ok(DisplayInside::FlowRoot),
                            _ => Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
                        }
                    })
                },
                &mut |context, input| {
                    parse_item_if_missing(context, input, &mut item, |_, input| {
                        input.expect_ident_matching("list-item")?;
                        Ok(())
                    })
                },
            ],
        );
        Ok(DisplayListItem { outside, inside })
    }
}

#[derive(Clone)]
pub enum DisplayInternal {
    TableRowGroup,
    TableHeaderGroup,
    TableFooterGroup,
    TableRow,
    TableCell,
    TableColumnGroup,
    TableColumn,
    TableCaption,
    RubyBase,
    RubyText,
    RubyBaseContainer,
    RubyTextContainer,
}

property_keywords_impl! { DisplayInternal,
    DisplayInternal::TableRowGroup, "table-row-group",
    DisplayInternal::TableHeaderGroup, "table-header-group",
    DisplayInternal::TableFooterGroup, "table-footer-group",
    DisplayInternal::TableRow, "table-row",
    DisplayInternal::TableCell, "table-cell",
    DisplayInternal::TableColumnGroup, "table-column-group",
    DisplayInternal::TableColumn, "table-column",
    DisplayInternal::TableCaption, "table-caption",
    DisplayInternal::RubyBase, "ruby-base",
    DisplayInternal::RubyText, "ruby-text",
    DisplayInternal::RubyBaseContainer, "ruby-base-container",
    DisplayInternal::RubyTextContainer, "ruby-text-container",
}

#[derive(Clone)]
pub enum DisplayBox {
    Contents,
    None,
}

property_keywords_impl! { DisplayBox,
    DisplayBox::Contents, "contents",
    DisplayBox::None, "none",
}

#[derive(Clone)]
pub enum DisplayLegacy {
    InlineBlock,
    InlineTable,
    InlineFlex,
    InlineGrid,
}

property_keywords_impl! { DisplayLegacy,
    DisplayLegacy::InlineBlock, "inline-block",
    DisplayLegacy::InlineTable, "inline-table",
    DisplayLegacy::InlineFlex, "inline-flex",
    DisplayLegacy::InlineGrid, "inline-grid",
}

#[derive(Clone)]
pub enum Display {
    Basic(DisplayBasic),
    ListItem(DisplayListItem),
    Internal(DisplayInternal),
    Box(DisplayBox),
    Legacy(DisplayLegacy),
}

impl Display {
    pub fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i>> {
        input
            .try_parse(|input| {
                let basic = DisplayBasic::parse(context, input)?;
                Ok(Display::Basic(basic))
            })
            .or_else(|_err: ParseError<'i>| {
                input.try_parse(|input| {
                    let item = DisplayListItem::parse(context, input)?;
                    Ok(Display::ListItem(item))
                })
            })
            .or_else(|_err: ParseError<'i>| {
                input.try_parse(|input| {
                    let internal = DisplayInternal::parse(input)?;
                    Ok(Display::Internal(internal))
                })
            })
            .or_else(|_err: ParseError<'i>| {
                input.try_parse(|input| {
                    let box_ = DisplayBox::parse(input)?;
                    Ok(Display::Box(box_))
                })
            })
            .or_else(|_err: ParseError<'i>| {
                let legacy = DisplayLegacy::parse(input)?;
                Ok(Display::Legacy(legacy))
            })
    }
}

pub fn parse_declared<'i, 't>(
    context: &ParserContext,
    input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
    Display::parse(context, input).map(PropertyDeclaration::Display)
}
