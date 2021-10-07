use std::rc::{Rc, Weak};

use dom::global_scope::NodeRef;

pub struct BaseBox {
	dom_node: NodeRef,
	formatting_context: Rc<VisualFormattingContext>,
	stacking_context: Rc<StackingContext>,
}

/// Block-level box is also a block container
/// https://www.w3.org/TR/CSS22/visuren.html#block-boxes
pub struct BlockBox {
	base: BaseBox,
	children: Vec<VisualBox>, // only BlockBox and AnonymousBox
}

/// Inline-level box is also a block container
/// https://www.w3.org/TR/CSS22/visuren.html#inline-boxes
pub struct InlineBox {
	base: BaseBox,
	children: Vec<VisualBox>, // only InlineBox and AnonymousBox
}

pub enum VisualBox {
	BlockBox(BlockBox),
	InlineBox(InlineBox),
	AnonymousBox(Box<VisualBox>),
}

pub enum FormattingContextType {
	BlockFormattingContext,
	InlineFormattingContext,
}

pub struct VisualFormattingContext {
	pub formatting_context_type: FormattingContextType,
	pub established_by: Weak<VisualBox>,
}

pub struct StackingContext {
	pub z_index: i32,
	pub generated_by: Weak<VisualBox>,
	pub children: Vec<StackingContext>,
}

pub struct FlowTree {}
