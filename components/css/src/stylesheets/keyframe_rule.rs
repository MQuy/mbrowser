use std::fmt::Write;

use cssparser::{
	AtRuleParser, CowRcStr, DeclarationListParser, DeclarationParser, Parser, ParserState, QualifiedRuleParser,
	RuleListParser, SourceLocation, Token,
};

use super::css_rule::CssRuleType;
use super::rule_parser::{StyleParseErrorKind, VendorPrefix};
use super::stylesheet::ParserContext;
use crate::css_writer::{CssWriter, ToCss};
use crate::error_reporting::ContextualParseError;
use crate::parser::ParseError;
use crate::properties::declaration::{Importance, PropertyDeclaration};
use crate::properties::declaration_block::{PropertyDeclarationBlock, SourcePropertyDeclaration};
use crate::properties::property_id::PropertyId;
use crate::values::animation::KeyframesName;

/// A [`@keyframes`][keyframes] rule.
///
/// [keyframes]: https://drafts.csswg.org/css-animations/#keyframes
#[derive(Clone)]
pub struct KeyframesRule {
	/// The name of the current animation.
	pub name: KeyframesName,
	/// The keyframes specified for this CSS rule.
	pub keyframes: Vec<Keyframe>,
	/// Vendor prefix type the @keyframes has.
	pub vendor_prefix: Option<VendorPrefix>,
	/// The line and column of the rule's source code.
	pub source_location: SourceLocation,
}

impl ToCss for KeyframesRule {
	fn to_css<W>(&self, dest: &mut CssWriter<W>) -> core::fmt::Result
	where
		W: std::fmt::Write,
	{
		dest.write_str("@keyframes ")?;
		cssparser::ToCss::to_css(&self.name, &mut CssWriter::new(dest))?;
		dest.write_str(" {")?;
		for keyframe in self.keyframes.iter() {
			dest.write_str("\n")?;
			keyframe.to_css(&mut CssWriter::new(dest))?;
		}
		dest.write_str("\n}")
	}
}

/// A keyframe.
#[derive(Clone)]
pub struct Keyframe {
	/// The selector this keyframe was specified from.
	pub selector: KeyframeSelector,

	/// The declaration block that was declared inside this keyframe.
	///
	/// Note that `!important` rules in keyframes don't apply, but we keep this
	pub block: PropertyDeclarationBlock,

	/// The line and column of the rule's source code.
	pub source_location: SourceLocation,
}

impl ToCss for Keyframe {
	fn to_css<W>(&self, dest: &mut CssWriter<W>) -> core::fmt::Result
	where
		W: std::fmt::Write,
	{
		self.selector.to_css(&mut CssWriter::new(dest))?;
		dest.write_str(" {")?;
		// self.block.to_css(&mut CssWriter::new(dest))?;
		dest.write_str("\n}")
	}
}
/// A keyframes selector is a list of percentages or from/to symbols, which are
/// converted at parse time to percentages.
#[derive(Clone, Debug, PartialEq)]
pub struct KeyframeSelector(Vec<f32>);

impl KeyframeSelector {
	pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i>> {
		input
			.parse_comma_separated(|input| {
				let location = input.current_source_location();
				let token = input.next()?.clone();
				match token {
					Token::Ident(ref ident) if ident.as_ref().eq_ignore_ascii_case("from") => Ok(0.),
					Token::Ident(ref ident) if ident.as_ref().eq_ignore_ascii_case("to") => Ok(1.),
					Token::Percentage {
						unit_value: percentage, ..
					} if percentage >= 0. && percentage <= 1. => Ok(percentage),
					_ => Err(location.new_unexpected_token_error(token.clone())),
				}
			})
			.map(|vec| {
				let mut newvec = vec.clone();
				newvec.sort_by(|a, b| a.partial_cmp(b).unwrap());
				newvec.dedup();
				newvec
			})
			.map(KeyframeSelector)
	}
}

impl ToCss for KeyframeSelector {
	fn to_css<W>(&self, dest: &mut CssWriter<W>) -> core::fmt::Result
	where
		W: Write,
	{
		self.0
			.iter()
			.map(|percentage| match *percentage {
				value => dest.write_str(&std::format!("{}%", value * 100.0)),
			})
			.collect()
	}
}

struct KeyframeListParser<'a> {
	context: &'a ParserContext<'a>,
	declarations: &'a mut SourcePropertyDeclaration,
}

impl<'a, 'i> AtRuleParser<'i> for KeyframeListParser<'a> {
	type AtRule = Keyframe;
	type Error = StyleParseErrorKind<'i>;
	type PreludeBlock = ();
	type PreludeNoBlock = ();
}

impl<'a, 'i> QualifiedRuleParser<'i> for KeyframeListParser<'a> {
	type Error = StyleParseErrorKind<'i>;
	type Prelude = KeyframeSelector;
	type QualifiedRule = Keyframe;

	fn parse_prelude<'t>(&mut self, input: &mut Parser<'i, 't>) -> Result<Self::Prelude, ParseError<'i>> {
		let start_position = input.position();
		KeyframeSelector::parse(input).map_err(|e| {
			let location = e.location;
			let error = ContextualParseError::InvalidKeyframeRule(input.slice_from(start_position), e.clone());
			self.context.log_css_error(location, error);
			e
		})
	}

	fn parse_block<'t>(
		&mut self,
		selector: Self::Prelude,
		start: &ParserState,
		input: &mut Parser<'i, 't>,
	) -> Result<Self::QualifiedRule, ParseError<'i>> {
		let context =
			ParserContext::new_with_rule_type(self.context, CssRuleType::Keyframe, self.context.namespaces.unwrap());

		let parser = KeyframeDeclarationParser {
			context: &context,
			declarations: self.declarations,
		};
		let mut iter = DeclarationListParser::new(input, parser);
		let mut block = PropertyDeclarationBlock::new();
		while let Some(declaration) = iter.next() {
			match declaration {
				Ok(()) => {
					block.extend(iter.parser.declarations.drain(), Importance::Normal);
				},
				Err((error, slice)) => {
					iter.parser.declarations.clear();
					let location = error.location;
					let error = ContextualParseError::UnsupportedKeyframePropertyDeclaration(slice, error);
					context.log_css_error(location, error);
				},
			}
			// `parse_important` is not called here, `!important` is not allowed in keyframe blocks.
		}
		Ok(Keyframe {
			selector,
			block,
			source_location: start.source_location(),
		})
	}
}

struct KeyframeDeclarationParser<'a, 'b: 'a> {
	context: &'a ParserContext<'b>,
	declarations: &'a mut SourcePropertyDeclaration,
}

/// Default methods reject all at rules.
impl<'a, 'b, 'i> AtRuleParser<'i> for KeyframeDeclarationParser<'a, 'b> {
	type AtRule = ();
	type Error = StyleParseErrorKind<'i>;
	type PreludeBlock = ();
	type PreludeNoBlock = ();
}

impl<'a, 'b, 'i> DeclarationParser<'i> for KeyframeDeclarationParser<'a, 'b> {
	type Declaration = ();
	type Error = StyleParseErrorKind<'i>;

	fn parse_value<'t>(&mut self, name: CowRcStr<'i>, input: &mut Parser<'i, 't>) -> Result<(), ParseError<'i>> {
		let id = match PropertyId::parse(&name, self.context) {
			Ok(id) => id,
			Err(()) => {
				return Err(input.new_custom_error(StyleParseErrorKind::UnknownProperty(name)));
			},
		};

		PropertyDeclaration::parse_into(self.declarations, id, self.context, input)?;

		// In case there is still unparsed text in the declaration, we should
		// roll back.
		input.expect_exhausted()?;

		Ok(())
	}
}

/// Parses a keyframe list from CSS input.
pub fn parse_keyframe_list(context: &ParserContext, input: &mut Parser) -> Vec<Keyframe> {
	let mut declarations = SourcePropertyDeclaration::new();
	RuleListParser::new_for_nested_rule(
		input,
		KeyframeListParser {
			context,
			declarations: &mut declarations,
		},
	)
	.filter_map(Result::ok)
	.collect()
}
