use std::ops::Deref;

use css::selectors::select::Selectors;
use css::stylist::{Rule, Stylist};
use dom::global_scope::NodeRef;
use selectors::context::{MatchingContext, MatchingMode};
use selectors::matching::{matches_selector, ElementSelectorFlags};

use crate::applicable_declaration_block::ApplicableDeclarationBlock;

pub fn collect_rules(element: NodeRef, stylist: &Stylist) -> Vec<ApplicableDeclarationBlock> {
	let mut matching_context =
		MatchingContext::new(MatchingMode::Normal, None, None, stylist.quirks_mode());

	// Apply the selector flags. We should be in sequential mode
	// already, so we can directly apply the parent flags.
	let mut set_selector_flags = |element: &NodeRef, flags: ElementSelectorFlags| {
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

	let mut applicable_declarations = Vec::with_capacity(1);
	if let Some(style) = element.style_attribute().borrow().deref() {
		applicable_declarations.push(ApplicableDeclarationBlock::from_style(style));
	}
	collect_from_origin(
		&element,
		&mut applicable_declarations,
		stylist.author_cascade_data().rules(),
		&mut matching_context,
		&mut set_selector_flags,
	);
	applicable_declarations
}

fn collect_from_origin<F>(
	element: &NodeRef,
	applicable_declarations: &mut Vec<ApplicableDeclarationBlock>,
	rules: &Vec<Rule>,
	context: &mut MatchingContext<Selectors>,
	flags_setter: &mut F,
) where
	F: FnMut(&NodeRef, ElementSelectorFlags),
{
	for rule in rules {
		if matches_selector(
			&rule.selector,
			0,
			Some(&rule.hashes),
			element,
			context,
			flags_setter,
		) {
			applicable_declarations.push(ApplicableDeclarationBlock::from_rule(rule));
		}
	}
}
