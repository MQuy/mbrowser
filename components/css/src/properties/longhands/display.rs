use cssparser::{match_ignore_ascii_case, Parser, ToCss, Token, _cssparser_internal_to_lowercase};

use crate::computed_values::StyleContext;
use crate::css_writer::write_elements;
use crate::parser::{parse_in_any_order, parse_item_if_missing, ParseError};
use crate::properties::declaration::{property_keywords_impl, PropertyDeclaration};
use crate::str::convert_options_to_string;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum DisplayInside {
	Flow,
	FlowRoot,
	Table,
	Flex,
	Grid,
	Ruby,
}

#[derive(Clone, Debug)]
pub struct DisplayBasic {
	outside: Option<DisplayOutside>,
	inside: Option<DisplayInside>,
}

impl DisplayBasic {
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let mut outside = None;
		let mut inside = None;
		parse_in_any_order(
			input,
			&mut [
				&mut |input| {
					parse_item_if_missing(input, &mut outside, &mut |_, input| {
						DisplayOutside::parse(input)
					})
				},
				&mut |input| {
					parse_item_if_missing(input, &mut inside, &mut |_, input| {
						DisplayInside::parse(input)
					})
				},
			],
		);
		if outside.is_none() && inside.is_none() {
			return Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError));
		}
		Ok(DisplayBasic { outside, inside })
	}
}

impl ToCss for DisplayBasic {
	fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
	where
		W: std::fmt::Write,
	{
		let outside = self.outside.as_ref().map(|v| v.to_css_string());
		let inside = self.inside.as_ref().map(|v| v.to_css_string());
		write_elements(dest, &[outside.as_deref(), inside.as_deref()], ' ')
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

#[derive(Clone, Debug)]
pub struct DisplayListItem {
	outside: Option<DisplayOutside>,
	inside: Option<DisplayInside>, // only allow flow, flow-root
}

impl DisplayListItem {
	pub fn parse<'i, 't>(
		_context: &ParserContext,
		input: &mut Parser<'i, 't>,
	) -> Result<Self, ParseError<'i>> {
		let mut outside = None;
		let mut inside = None;
		let mut item = None;
		parse_in_any_order(
			input,
			&mut [
				&mut |input| {
					parse_item_if_missing(input, &mut outside, &mut |_, input| {
						DisplayOutside::parse(input)
					})
				},
				&mut |input| {
					parse_item_if_missing(input, &mut inside, &mut |_, input| {
						let location = input.current_source_location();
						let ident = input.expect_ident()?;
						match_ignore_ascii_case! { ident,
							"flow" => Ok(DisplayInside::Flow),
							"flow-root" => Ok(DisplayInside::FlowRoot),
							_ => Err(location.new_custom_error(StyleParseErrorKind::UnexpectedValue(ident.clone())))
						}
					})
				},
				&mut |input| {
					parse_item_if_missing(input, &mut item, &mut |_, input| {
						input.expect_ident_matching("list-item")?;
						Ok(())
					})
				},
			],
		);
		if item.is_none() {
			return Err(input.new_custom_error(StyleParseErrorKind::UnspecifiedError));
		}
		Ok(DisplayListItem { outside, inside })
	}
}

impl ToCss for DisplayListItem {
	fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
	where
		W: std::fmt::Write,
	{
		let outside = self.outside.as_ref().map(|v| v.to_css_string());
		let inside = self.inside.as_ref().map(|v| v.to_css_string());
		dest.write_str(&convert_options_to_string(
			vec![outside, inside, Some("list-item".to_string())],
			" ",
		))
	}
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum DisplayBox {
	Contents,
	None,
}

property_keywords_impl! { DisplayBox,
	DisplayBox::Contents, "contents",
	DisplayBox::None, "none",
}

#[derive(Clone, Debug)]
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

/// https://drafts.csswg.org/css-display/#the-display-properties
#[derive(Clone, Debug)]
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
				let item = DisplayListItem::parse(context, input)?;
				Ok(Display::ListItem(item))
			})
			.or_else(|_err: ParseError<'i>| {
				input.try_parse(|input| {
					let basic = DisplayBasic::parse(context, input)?;
					Ok(Display::Basic(basic))
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

impl ToCss for Display {
	fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
	where
		W: std::fmt::Write,
	{
		match self {
			Display::Basic(basic) => basic.to_css(dest),
			Display::ListItem(list) => list.to_css(dest),
			Display::Internal(internal) => internal.to_css(dest),
			Display::Box(box_) => box_.to_css(dest),
			Display::Legacy(legacy) => legacy.to_css(dest),
		}
	}
}

pub fn cascade_property<'a>(declaration: Option<&PropertyDeclaration>, context: &'a StyleContext) {
	todo!()
}

pub fn parse_declared<'i, 't>(
	context: &ParserContext,
	input: &mut Parser<'i, 't>,
) -> Result<PropertyDeclaration, ParseError<'i>> {
	Display::parse(context, input).map(PropertyDeclaration::Display)
}
