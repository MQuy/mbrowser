use std::any::Any;
use std::cell::{RefCell, RefMut};
use std::rc::{Rc, Weak};

use common::not_reached;
use css::values::{Pixel, PIXEL_ZERO};

use super::dimension::BoxDimension;
use super::formatting_context::{FormattingContext, FormattingContextType};

pub trait Box {
	fn add_child(&self, child: Rc<dyn Box>);

	fn formatting_context(&self) -> Rc<FormattingContext>;

	fn set_formatting_context(&self, formatting_context: Rc<FormattingContext>);

	fn formatting_context_type(&self) -> FormattingContextType;

	fn get_first_child(&self) -> Option<Rc<dyn Box>>;

	fn get_last_child(&self) -> Option<Rc<dyn Box>>;

	fn children(&self) -> Vec<Rc<dyn Box>>;

	fn parent(&self) -> Option<Rc<dyn Box>>;

	fn set_parent(&self, value: Option<Rc<dyn Box>>);

	fn ancestors(&self) -> SimpleBoxIterator;

	fn containing_block(&self) -> Option<Rc<dyn Box>>;

	fn set_containing_block(&self, value: Option<Rc<dyn Box>>);

	fn size(&self) -> RefMut<'_, BoxDimension>;

	fn get_total_width(&self) -> Pixel;

	fn get_total_height(&self) -> Pixel;

	fn is_block_container(&self) -> bool;

	fn compute_horizontal_used_value(&self);

	fn compute_vertical_used_value(&self);

	fn class(&self) -> BoxClass;

	fn as_any(&self) -> &dyn Any;
}

pub struct BaseBox {
	pub formatting_context: RefCell<Rc<FormattingContext>>,
	pub children: RefCell<Vec<Rc<dyn Box>>>,
	pub parent: RefCell<Option<Weak<dyn Box>>>,
	pub containing_block: RefCell<Option<Weak<dyn Box>>>,
	pub size: RefCell<BoxDimension>,
}

impl BaseBox {
	pub fn new(formatting_context: Rc<FormattingContext>) -> Self {
		BaseBox {
			parent: RefCell::new(Default::default()),
			formatting_context: RefCell::new(formatting_context),
			children: RefCell::new(Default::default()),
			containing_block: RefCell::new(Default::default()),
			size: RefCell::new(Default::default()),
		}
	}

	#[inline]
	pub fn add_child(&self, child: Rc<dyn Box>) {
		self.children.borrow_mut().push(child)
	}

	#[inline]
	pub fn formatting_context(&self) -> Rc<FormattingContext> {
		self.formatting_context.borrow().clone()
	}

	#[inline]
	pub fn set_formatting_context(&self, formatting_context: Rc<FormattingContext>) {
		self.formatting_context.replace(formatting_context);
	}

	#[inline]
	pub fn formatting_context_type(&self) -> FormattingContextType {
		self.formatting_context.borrow().formatting_context_type
	}

	#[inline]
	pub fn get_first_child(&self) -> Option<Rc<dyn Box>> {
		self.children.borrow().first().map(|value| value.clone())
	}

	#[inline]
	pub fn get_last_child(&self) -> Option<Rc<dyn Box>> {
		self.children.borrow().last().map(|value| value.clone())
	}

	#[inline]
	pub fn children(&self) -> Vec<Rc<dyn Box>> {
		self.children.borrow().clone()
	}

	#[inline]
	pub fn parent(&self) -> Option<Rc<dyn Box>> {
		match self.parent.borrow().as_ref() {
			Some(value) => value.upgrade(),
			None => None,
		}
	}

	#[inline]
	pub fn set_parent(&self, value: Option<Rc<dyn Box>>) {
		self.parent
			.replace(value.as_ref().map(|v| Rc::downgrade(v)));
	}

	#[inline]
	pub fn containing_block(&self) -> Option<Rc<dyn Box>> {
		match self.containing_block.borrow().as_ref() {
			Some(value) => value.upgrade(),
			None => None,
		}
	}

	#[inline]
	pub fn set_containing_block(&self, value: Option<Rc<dyn Box>>) {
		self.containing_block
			.replace(value.as_ref().map(|v| Rc::downgrade(v)));
	}

	#[inline]
	pub fn size(&self) -> RefMut<'_, BoxDimension> {
		self.size.borrow_mut()
	}
}

// Anonymous box is always anonymous block box
pub struct AnonymousBox {
	base: BaseBox,
}

impl AnonymousBox {
	pub fn new(formatting_context: Rc<FormattingContext>) -> Self {
		AnonymousBox {
			base: BaseBox::new(formatting_context),
		}
	}
}

impl Box for AnonymousBox {
	fn add_child(&self, child: Rc<dyn Box>) {
		self.base.add_child(child)
	}

	fn formatting_context(&self) -> Rc<FormattingContext> {
		self.base.formatting_context()
	}

	fn set_formatting_context(&self, formatting_context: Rc<FormattingContext>) {
		self.base.set_formatting_context(formatting_context);
	}

	fn formatting_context_type(&self) -> FormattingContextType {
		self.base.formatting_context_type()
	}

	fn get_first_child(&self) -> Option<Rc<dyn Box>> {
		self.base.get_first_child()
	}

	fn get_last_child(&self) -> Option<Rc<dyn Box>> {
		self.base.get_last_child()
	}

	fn children(&self) -> Vec<Rc<dyn Box>> {
		self.base.children()
	}

	fn parent(&self) -> Option<Rc<dyn Box>> {
		self.base.parent()
	}

	fn set_parent(&self, value: Option<Rc<dyn Box>>) {
		self.base.set_parent(value);
	}

	fn ancestors(&self) -> SimpleBoxIterator {
		SimpleBoxIterator::new(self.parent(), &|n: &Rc<dyn Box>| n.parent())
	}

	fn containing_block(&self) -> Option<Rc<dyn Box>> {
		self.base.containing_block()
	}

	fn set_containing_block(&self, value: Option<Rc<dyn Box>>) {
		self.base.set_containing_block(value)
	}

	fn size(&self) -> RefMut<'_, BoxDimension> {
		self.base.size()
	}

	fn get_total_width(&self) -> Pixel {
		self.base.size.borrow().width
	}

	fn get_total_height(&self) -> Pixel {
		self.base.size.borrow().height
	}

	fn is_block_container(&self) -> bool {
		false
	}

	fn compute_horizontal_used_value(&self) {
		let containing_width = self
			.containing_block()
			.expect("has to have a containing block")
			.size()
			.width;
		self.size().set_width(containing_width)
	}

	fn compute_vertical_used_value(&self) {
		let height = BoxClass::get_total_children_height(self);
		self.size().set_height(height);
	}

	fn class(&self) -> BoxClass {
		BoxClass::Anonymous
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}

#[derive(Debug, PartialEq)]
pub enum BoxClass {
	Inline,
	Block,
	Anonymous,
}

impl BoxClass {
	pub fn new_with_formatting_context<F>(
		formatting_context_type: FormattingContextType,
		setup: F,
	) -> Rc<dyn Box>
	where
		F: FnOnce(Rc<FormattingContext>) -> Rc<dyn Box>,
	{
		let formatting_context = Rc::new(FormattingContext::new(formatting_context_type));
		let level_box = setup(formatting_context.clone());
		formatting_context.set_established_by(level_box.clone());
		level_box
	}

	pub fn append_child(source: Rc<dyn Box>, child: Rc<dyn Box>) {
		let child = match source.formatting_context_type() {
			FormattingContextType::BlockFormattingContext if child.class() == BoxClass::Inline => {
				let last_child = source.get_last_child();
				if let Some(last_child) = last_child {
					if last_child.class() == BoxClass::Anonymous {
						BoxClass::add_child(last_child, child);
						return;
					}
				}

				let anonymous_box = BoxClass::new_with_formatting_context(
					FormattingContextType::InlineFormattingContext,
					|formatting_context: Rc<FormattingContext>| {
						Rc::new(AnonymousBox::new(formatting_context))
					},
				);
				child.set_formatting_context(anonymous_box.formatting_context());
				BoxClass::add_child(anonymous_box.clone(), child);
				anonymous_box
			},
			FormattingContextType::InlineFormattingContext if child.class() == BoxClass::Block => {
				not_reached!()
			},
			_ => child,
		};
		BoxClass::add_child(source, child);
	}

	pub fn add_child(source: Rc<dyn Box>, child: Rc<dyn Box>) {
		source.add_child(child.clone());
		child.set_parent(Some(source));
	}

	pub fn get_total_children_intrinsic_width(source: &dyn Box) -> Pixel {
		let mut total_children_width = PIXEL_ZERO;
		match source.formatting_context_type() {
			FormattingContextType::BlockFormattingContext => {
				for child in source.children() {
					total_children_width = child.size().intrinsic_width.max(total_children_width);
				}
			},
			FormattingContextType::InlineFormattingContext => {
				for child in source.children() {
					total_children_width += child.size().intrinsic_width;
				}
			},
		}
		total_children_width
	}

	pub fn get_total_children_height(source: &dyn Box) -> Pixel {
		assert!(source.size().width != PIXEL_ZERO);

		let mut total_children_height = PIXEL_ZERO;
		match source.formatting_context_type() {
			FormattingContextType::BlockFormattingContext => {
				// TODO: Support margin collapse
				for child in source.children() {
					total_children_height += child.get_total_height();
				}
			},
			FormattingContextType::InlineFormattingContext => {
				// TODO: Support multilines
				for child in source.children() {
					total_children_height = total_children_height.max(child.get_total_height());
				}
			},
		};
		total_children_height
	}

	pub fn set_containing_box(source: Rc<dyn Box>) {
		let mut containing_block = None;
		for ancestor in source.ancestors() {
			// TODO: include box which establishes a new formatting context
			// https://www.w3.org/TR/CSS22/visudet.html#containing-block-details
			if ancestor.is_block_container() {
				containing_block = Some(ancestor);
				break;
			}
		}
		if let Some(containing_block) = containing_block {
			source.set_containing_block(Some(containing_block.clone()));
			// during constructing box tree, there might be anonymous boxes which are added (as its parent) to ensure https://www.w3.org/TR/CSS22/visuren.html#anonymous-block-level
			for ancestor in source.ancestors() {
				match ancestor.class() {
					BoxClass::Anonymous if ancestor.containing_block().is_none() => {
						ancestor.set_containing_block(Some(containing_block.clone()))
					},
					_ => break,
				}
			}
		} else {
			panic!("one of box's ancestors must be its containing box");
		}
	}
}

pub struct SimpleBoxIterator<'a> {
	current: Option<Rc<dyn Box>>,
	next_node: &'a dyn Fn(&Rc<dyn Box>) -> Option<Rc<dyn Box>>,
}

impl<'a> SimpleBoxIterator<'a> {
	pub fn new(
		current: Option<Rc<dyn Box>>,
		next_node: &'a dyn Fn(&Rc<dyn Box>) -> Option<Rc<dyn Box>>,
	) -> Self {
		SimpleBoxIterator { current, next_node }
	}
}

impl<'a> Iterator for SimpleBoxIterator<'a> {
	type Item = Rc<dyn Box>;

	fn next(&mut self) -> Option<Self::Item> {
		let current = self.current.take();
		self.current = current.as_ref().and_then(|c| (self.next_node)(c));
		current
	}
}
