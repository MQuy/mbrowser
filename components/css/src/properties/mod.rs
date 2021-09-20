pub mod custom_properties;
pub mod declaration;
pub mod declaration_block;
pub mod longhand_id;
pub mod longhands;
pub mod property_id;
pub mod shorthand_id;
pub mod shorthands;

bitflags! {
	#[repr(C)]
	/// https://www.w3.org/TR/CSS22/cascade.html#specificity
	pub struct SelectorSpecificity: u32 {
		const ELEMENT = 1 << 0;
		const ATTRIBUTE = 1 << 10;
		const ID = 1 << 20;
		const STYLE = 1 << 30;
		const IMPORTANT = 1 << 31;
	}
}
