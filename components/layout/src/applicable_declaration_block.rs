use css::properties::declaration_block::PropertyDeclarationBlock;
use css::stylesheets::style_rule::StyleRule;
use css::stylist::Rule;

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
}

pub enum StyleSource {
	StyleRule(StyleRule),
	DeclarationBlock(PropertyDeclarationBlock),
}
