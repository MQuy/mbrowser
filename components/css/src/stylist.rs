use common::not_supported;
use selectors::context::QuirksMode;
use selectors::parser::{AncestorHashes, Selector};

use crate::selectors::select::Selectors;
use crate::stylesheets::css_rule::CssRule;
use crate::stylesheets::origin::Origin;
use crate::stylesheets::style_rule::StyleRule;
use crate::stylesheets::stylesheet::Stylesheet;

pub struct Stylist {
	user_agent: CascadeData,
	author: CascadeData,
	quirks_mode: QuirksMode,
}

impl Stylist {
	pub fn new(quirks_mode: QuirksMode) -> Self {
		Stylist {
			user_agent: Default::default(),
			author: Default::default(),
			quirks_mode,
		}
	}

	pub fn quirks_mode(&self) -> QuirksMode {
		self.quirks_mode
	}

	pub fn author_cascade_data(&self) -> &CascadeData {
		&self.author
	}

	pub fn add_stylesheet(&mut self, stylesheet: &Stylesheet, origin: Origin) {
		match origin {
			Origin::UserAgent => self.user_agent.add_stylesheet(stylesheet),
			Origin::Author => self.author.add_stylesheet(stylesheet),
			Origin::User => not_supported!(),
		}
	}
}

pub struct CascadeData {
	rules: Vec<Rule>,
	rules_source_order: u32,
}

impl CascadeData {
	pub fn add_stylesheet(&mut self, stylesheet: &Stylesheet) {
		for css_rule in &stylesheet.rules {
			match css_rule {
				CssRule::Style(style) => {
					for selector in &style.selectors.0 {
						let hashes = AncestorHashes::new(&selector, stylesheet.quirks_mode);
						let rule = Rule {
							selector: selector.clone(),
							hashes,
							source_order: self.rules_source_order,
							style_rule: style.clone(),
						};
						self.rules_source_order += 1;
						self.rules.push(rule);
					}
				},
				_ => todo!(),
			}
		}
	}

	pub fn rules(&self) -> &Vec<Rule> {
		&self.rules
	}
}

impl Default for CascadeData {
	fn default() -> Self {
		Self {
			rules: Default::default(),
			rules_source_order: Default::default(),
		}
	}
}

#[derive(Debug)]
pub struct Rule {
	pub selector: Selector<Selectors>,
	pub hashes: AncestorHashes,
	pub source_order: u32,
	pub style_rule: StyleRule,
}
