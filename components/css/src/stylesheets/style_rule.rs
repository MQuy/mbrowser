use std::fmt::Write;

use cssparser::SourceLocation;
use selectors::SelectorList;

use crate::css_writer::ToCss;
use crate::properties::declaration_block::PropertyDeclarationBlock;
use crate::selectors::select::Selectors;

/// A style rule, with selectors and declarations.
#[derive(Clone, Debug)]
pub struct StyleRule {
	/// The list of selectors in this rule.
	pub selectors: SelectorList<Selectors>,
	/// The declaration block with the properties it contains.
	pub block: PropertyDeclarationBlock,
	/// The location in the sheet where it was found.
	pub source_location: SourceLocation,
}

impl ToCss for StyleRule {
	fn to_css<W>(&self, dest: &mut crate::css_writer::CssWriter<W>) -> core::fmt::Result
	where
		W: std::fmt::Write,
	{
		use cssparser::ToCss;
		self.selectors.to_css(dest)?;
		dest.write_str(" {\n")?;
		self.block.to_css(dest)?;
		dest.write_str("}")
	}
}
