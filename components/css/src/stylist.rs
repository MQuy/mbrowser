use std::rc::Rc;

use selectors::context::QuirksMode;
use selectors::parser::{AncestorHashes, Selector};

use crate::selectors::select::Selectors;
use crate::stylesheets::css_rule::CssRule;
use crate::stylesheets::style_rule::StyleRule;
use crate::stylesheets::stylesheet::Stylesheet;

pub struct Stylist {
	stylesheets: Vec<Rc<Stylesheet>>,
	rules: Vec<Rule>,
	rules_source_order: u32,
	quirks_mode: QuirksMode,
}

impl Stylist {
	pub fn new(quirks_mode: QuirksMode) -> Self {
		Stylist {
			stylesheets: Vec::with_capacity(1),
			rules: vec![],
			rules_source_order: 0,
			quirks_mode,
		}
	}

	pub fn get_rules(&self) -> &Vec<Rule> {
		&self.rules
	}

	pub fn get_quirks_mode(&self) -> QuirksMode {
		self.quirks_mode
	}

	pub fn add_stylesheet(&mut self, stylesheet: Rc<Stylesheet>) {
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
		self.stylesheets.push(stylesheet)
	}
}

pub struct Rule {
	pub selector: Selector<Selectors>,
	pub hashes: AncestorHashes,
	pub source_order: u32,
	pub style_rule: StyleRule,
}
