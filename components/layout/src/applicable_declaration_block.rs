use css::properties::declaration_block::PropertyDeclarationBlock;
use css::stylesheets::style_rule::StyleRule;
use css::stylist::Rule;

#[derive(Debug)]
pub struct ApplicableDeclarationBlock {
	pub source: StyleSource,
	pub specificity: u32,
}

impl ApplicableDeclarationBlock {
	pub fn from_rule(rule: &Rule) -> Self {
		Self {
			specificity: rule.selector.specificity(),
			source: StyleSource::StyleRule(rule.style_rule.clone()),
		}
	}

	pub fn from_style(declaration: &PropertyDeclarationBlock) -> Self {
		Self {
			specificity: 1 << 30,
			source: StyleSource::DeclarationBlock(declaration.clone()),
		}
	}
}

#[derive(Debug)]
pub enum StyleSource {
	StyleRule(StyleRule),
	DeclarationBlock(PropertyDeclarationBlock),
}
