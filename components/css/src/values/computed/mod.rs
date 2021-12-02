pub mod length;

macro_rules! from_non_inherited_property {
	($declaration: tt, $inherited_value: expr, $initial_value: expr, $longhand_id: expr, $pattern: pat => $then: expr) => {
		match $declaration {
			Some($declaration) => match $declaration {
				$pattern => $then,
				PropertyDeclaration::CSSWideKeyword(WideKeywordDeclaration { id, keyword }) if *id == $longhand_id => {
					match keyword {
						CSSWideKeyword::Initial | CSSWideKeyword::Unset => $initial_value,
						CSSWideKeyword::Inherit => $inherited_value,
						CSSWideKeyword::Revert => unreachable!(),
					}
				},
				_ => unreachable!(),
			},
			None => $initial_value,
		}
	};
}
pub(crate) use from_non_inherited_property;

macro_rules! from_inherited_property {
	($declaration: tt, $inherited_value: expr, $initial_value: expr, $longhand_id: expr, $pattern: pat => $then: expr) => {
		match $declaration {
			Some($declaration) => match $declaration {
				$pattern => $then,
				PropertyDeclaration::CSSWideKeyword(WideKeywordDeclaration { id, keyword }) if *id == $longhand_id => {
					match keyword {
						CSSWideKeyword::Initial => $initial_value,
						CSSWideKeyword::Inherit | CSSWideKeyword::Unset => $inherited_value,
						CSSWideKeyword::Revert => unreachable!(),
					}
				},
				_ => unreachable!(),
			},
			None => $inherited_value,
		}
	};
}
pub(crate) use from_inherited_property;
