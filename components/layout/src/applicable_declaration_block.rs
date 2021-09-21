use css::properties::declaration_block::PropertyDeclarationBlock;
use css::properties::SelectorSpecificity;
use css::stylesheets::origin::Origin;
use css::stylesheets::style_rule::StyleRule;
use css::stylist::Rule;

#[derive(Debug)]
pub struct ApplicableDeclarationBlock {
	pub source: StyleSource,
	pub specificity: u32,
	pub origin: Origin,
}

impl ApplicableDeclarationBlock {
	pub fn from_rule(rule: &Rule, origin: Origin) -> Self {
		Self {
			origin,
			specificity: rule.selector.specificity(),
			source: StyleSource::StyleRule(rule.style_rule.clone()),
		}
	}

	pub fn from_style(declaration: &PropertyDeclarationBlock) -> Self {
		Self {
			origin: Origin::Author,
			specificity: SelectorSpecificity::STYLE.bits(),
			source: StyleSource::DeclarationBlock(declaration.clone()),
		}
	}
}

#[derive(Debug)]
pub enum StyleSource {
	StyleRule(StyleRule),
	DeclarationBlock(PropertyDeclarationBlock),
}
