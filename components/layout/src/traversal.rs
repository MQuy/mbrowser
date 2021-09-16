use css::stylist::Stylist;
use dom::element::Element;
use selectors::context::{MatchingContext, MatchingMode};
use selectors::matching::{matches_selector, ElementSelectorFlags};

use crate::applicable_declaration_block::ApplicableDeclarationBlock;

pub fn compute_values(element: &Element, stylist: &Stylist) {
	let mut matching_context =
		MatchingContext::new(MatchingMode::Normal, None, None, stylist.get_quirks_mode());

	// Apply the selector flags. We should be in sequential mode
	// already, so we can directly apply the parent flags.
	let mut set_selector_flags = |element: &Element, flags: ElementSelectorFlags| {
		let self_flags = flags.for_self();
		if !self_flags.is_empty() {
			element.insert_selector_flags(self_flags);
		}
		let parent_flags = flags.for_parent();
		if !parent_flags.is_empty() {
			if let Some(p) = element.parent_element() {
				p.insert_selector_flags(parent_flags);
			}
		}
	};

	let mut matching_rules = Vec::with_capacity(1);
	for rule in stylist.get_rules() {
		if matches_selector(
			&rule.selector,
			0,
			Some(&rule.hashes),
			element,
			&mut matching_context,
			&mut set_selector_flags,
		) {
			matching_rules.push(ApplicableDeclarationBlock::from_rule(rule));
		}
	}
}
