use std::collections::HashMap;
use std::rc::Rc;

use css::computed_values::{ComputedValues, PropertyCascade, StyleContext};
use css::properties::declaration::{PropertyDeclaration, WideKeywordDeclaration};
use css::properties::longhand_id::{LonghandId, LonghandIdPhaseIterator, PhaseOrder};
use css::properties::property_id::CSSWideKeyword;
use css::stylesheets::origin::Origin;
use dom::global_scope::GlobalScope;

use crate::applicable_declaration_block::StyleSource;
use crate::style_tree::StyleTreeNode;

pub fn cascade(style_node: Rc<StyleTreeNode>, parent_style: &ComputedValues) {
	let mut author_data: HashMap<LonghandId, PropertyCascade> = HashMap::new();
	let mut useragent_data: HashMap<LonghandId, PropertyCascade> = HashMap::new();
	let rules = style_node.rules.borrow();
	for declaration in rules.iter() {
		let block = match &declaration.source {
			StyleSource::StyleRule(style) => &style.block,
			StyleSource::DeclarationBlock(block) => block,
		};
		for (importance, property) in block.properties() {
			match declaration.origin {
				Origin::UserAgent => {
					cascade_in_origin(
						&mut useragent_data,
						property,
						importance,
						declaration.specificity,
					);
				},
				Origin::Author => {
					cascade_in_origin(
						&mut author_data,
						property,
						importance,
						declaration.specificity,
					);
				},
			}
		}
	}
	let mut computed_values = GlobalScope::get_or_init_computed_values(style_node.dom_node.id());
	let mut context = StyleContext {
		parent_style,
		author_data,
		useragent_data,
		computed_values: &mut computed_values,
	};
	apply_properties(LonghandId::ids(PhaseOrder::Early), &mut context);
	apply_properties(LonghandId::ids(PhaseOrder::Other), &mut context);

	let mut child = style_node.first_child.borrow().as_ref().map(|n| n.clone());
	loop {
		let noderef = if let Some(ref noderef) = child {
			noderef.clone()
		} else {
			break;
		};
		cascade(noderef.clone(), computed_values);
		child = if let Some(child) = noderef.next_sibling.borrow().as_ref() {
			child.upgrade()
		} else {
			None
		};
	}
}

fn cascade_in_origin<'a, 'b>(
	cascade_data: &'a mut HashMap<LonghandId, PropertyCascade<'b>>,
	property: &'b PropertyDeclaration,
	importance: bool,
	specificity: u32,
) {
	if let Some(cascade) = cascade_data.get(&property.longhand_id()) {
		if (cascade.importance && !importance) || (cascade.specificity > specificity) {
			return;
		}
	}
	cascade_data.insert(
		property.longhand_id(),
		PropertyCascade {
			specificity,
			importance,
			property,
		},
	);
}

fn get_declaration_from_useragent<'a>(
	cascade_data: &HashMap<LonghandId, PropertyCascade<'a>>,
	longhand_id: &LonghandId,
	unset: &'a PropertyDeclaration,
) -> Option<&'a PropertyDeclaration> {
	let useragent_property = cascade_data
		.get(longhand_id)
		.map(|cascade| cascade.property);
	if let Some(property) = useragent_property {
		Some(match property {
			PropertyDeclaration::CSSWideKeyword(WideKeywordDeclaration { keyword, .. })
				if *keyword == CSSWideKeyword::Revert =>
			{
				unset
			},
			value => value,
		})
	} else {
		None
	}
}

fn apply_properties<'a>(longhands_iter: LonghandIdPhaseIterator, context: &'a mut StyleContext) {
	for longhand_id in longhands_iter {
		let unset = PropertyDeclaration::CSSWideKeyword(WideKeywordDeclaration {
			id: longhand_id,
			keyword: CSSWideKeyword::Unset,
		});
		let author_property = context
			.author_data
			.get(&longhand_id)
			.map(|cascade| cascade.property);
		let declaration = if let Some(property) = author_property {
			match property {
				PropertyDeclaration::CSSWideKeyword(WideKeywordDeclaration { keyword, .. })
					if *keyword == CSSWideKeyword::Revert =>
				{
					get_declaration_from_useragent(&context.useragent_data, &longhand_id, &unset)
				},
				value => Some(value),
			}
		} else {
			get_declaration_from_useragent(&context.useragent_data, &longhand_id, &unset)
		};
		longhand_id.cascade(declaration, context)
	}
}
