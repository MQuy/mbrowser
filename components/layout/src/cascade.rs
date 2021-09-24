use std::collections::HashMap;
use std::rc::Rc;

use css::computed_values::{ComputedValues, PropertyCascade, StyleContext};
use css::properties::longhand_id::{LonghandId, LonghandIdPhaseIterator, PhaseOrder};
use css::stylesheets::origin::Origin;
use dom::global_scope::{GlobalScope, NodeRef};

use crate::applicable_declaration_block::StyleSource;
use crate::style_tree::StyleTreeNode;

/*
- create a hash with key=property name, value = (specificity, origin, important, property)
- loop through rules
	- fill in the hash
	- if exist, check specificity, origin and import to replace
- create a context with that hash, parent_style
- for each ComputedValues's name, call cascade with the context
 */
pub fn cascade(style_node: Rc<StyleTreeNode>, parent_style: Option<&ComputedValues>) {
	let mut cascade_data: HashMap<LonghandId, PropertyCascade> = HashMap::new();
	let rules = style_node.rules.borrow();
	for declaration in rules.iter() {
		let block = match &declaration.source {
			StyleSource::StyleRule(style) => &style.block,
			StyleSource::DeclarationBlock(block) => block,
		};
		for (importance, property) in block.properties() {
			if let Some(cascade) = cascade_data.get(&property.longhand_id()) {
				if (cascade.origin == Origin::Author && declaration.origin == Origin::UserAgent)
					|| !((cascade.origin == Origin::UserAgent
						&& declaration.origin == Origin::Author)
						|| (!cascade.importance && importance)
						|| (cascade.specificity > declaration.specificity))
				{
					continue;
				}
			}
			cascade_data.insert(
				property.longhand_id(),
				PropertyCascade {
					origin: declaration.origin,
					specificity: declaration.specificity,
					importance,
					property,
				},
			);
		}
	}
	let mut computed_values = GlobalScope::get_or_init_computed_values(style_node.dom_node.id());
	let context = StyleContext {
		parent_style,
		cascade_data,
		computed_values: &mut computed_values,
	};
	apply_rules(&context, style_node.dom_node.clone());

	let mut child = style_node.first_child.borrow().as_ref().map(|n| n.clone());
	loop {
		let noderef = if let Some(ref noderef) = child {
			noderef.clone()
		} else {
			break;
		};
		cascade(noderef.clone(), Some(computed_values));
		child = if let Some(child) = noderef.next_sibling.borrow().as_ref() {
			child.upgrade()
		} else {
			None
		};
	}
}

pub fn apply_rules<'a, 'b>(context: &'a StyleContext, node: NodeRef) {
	apply_properties(LonghandId::ids(PhaseOrder::Early), context);
	apply_properties(LonghandId::ids(PhaseOrder::Other), context);
}

pub fn apply_properties<'a>(longhands_iter: LonghandIdPhaseIterator, context: &'a StyleContext) {
	for longhand_id in longhands_iter {
		longhand_id.cascade(
			context
				.cascade_data
				.get(&longhand_id)
				.map(|cascade| cascade.property),
			context,
		);
	}
}
