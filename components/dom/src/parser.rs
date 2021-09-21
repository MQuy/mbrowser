use std::fmt::Display;
use std::rc::Rc;

use common::not_supported;
use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode as MarkupQuirksMode, TreeSink};
use html5ever::{Attribute, LocalName, QualName};
use log::debug;
use selectors::context::QuirksMode;

use crate::characterdata::CharacterData;
use crate::comment::Comment;
use crate::document::Document;
use crate::documenttype::DocumentType;
use crate::element::Element;
use crate::global_scope::add_to_global_scope;
use crate::inheritance::{downcast, upcast, Castable};
use crate::node::Node;
use crate::text::Text;
use crate::virtualmethods::vtable_for;
use crate::window::{CSSErrorReporter, Window};

pub struct DomParser {
	pub document: Rc<Document>,
	current_line: u64,
}

impl DomParser {
	pub fn new() -> Self {
		let document = Rc::new(Document::new(None));
		add_to_global_scope(upcast(document.clone()));
		let window = Window::new(document.clone(), CSSErrorReporter::new());
		document.set_window(Rc::new(window));
		Self {
			document,
			current_line: 0,
		}
	}
}

impl Display for DomParser {
	fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Ok(())
	}
}

impl TreeSink for DomParser {
	type Handle = Rc<Node>;
	type Output = Self;

	fn finish(self) -> Self::Output {
		self
	}

	fn parse_error(&mut self, msg: std::borrow::Cow<'static, str>) {
		debug!("Parse error: {}", msg);
	}

	fn get_document(&mut self) -> Self::Handle {
		upcast(self.document.clone())
	}

	fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> html5ever::ExpandedName<'a> {
		let elem = target.downcast::<Element>();
		html5ever::ExpandedName {
			ns: &elem.namespace(),
			local: &elem.local_name(),
		}
	}

	fn create_element(
		&mut self,
		name: html5ever::QualName,
		attrs: Vec<html5ever::Attribute>,
		_flags: ElementFlags,
	) -> Self::Handle {
		let element = create_element_for_token(name, attrs, self.document.clone());
		upcast(element)
	}

	fn create_comment(&mut self, text: html5ever::tendril::StrTendril) -> Self::Handle {
		let comment = Comment::create(String::from(text), self.document.clone());
		upcast(comment)
	}

	fn create_pi(
		&mut self,
		_target: html5ever::tendril::StrTendril,
		_data: html5ever::tendril::StrTendril,
	) -> Self::Handle {
		not_supported!()
	}

	fn append(&mut self, parent: &Self::Handle, child: NodeOrText<Self::Handle>) {
		insert(parent, None, child);
	}

	fn append_based_on_parent_node(
		&mut self,
		element: &Self::Handle,
		prev_element: &Self::Handle,
		child: NodeOrText<Self::Handle>,
	) {
		if element.parent_node().is_some() {
			self.append_before_sibling(element, child);
		} else {
			self.append(prev_element, child);
		}
	}

	fn append_doctype_to_document(
		&mut self,
		name: html5ever::tendril::StrTendril,
		public_id: html5ever::tendril::StrTendril,
		system_id: html5ever::tendril::StrTendril,
	) {
		let doctype = DocumentType::new(
			String::from(name),
			String::from(public_id),
			String::from(system_id),
			self.document.clone(),
		);
		upcast(self.document.clone())
			.append_child(upcast(Rc::new(doctype)))
			.expect("Appending failed");
	}

	fn mark_script_already_started(&mut self, _node: &Self::Handle) {
		todo!()
	}

	fn pop(&mut self, node: &Self::Handle) {
		vtable_for(&node).pop();
	}

	fn get_template_contents(&mut self, _target: &Self::Handle) -> Self::Handle {
		todo!()
	}

	fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
		x == y
	}

	fn set_quirks_mode(&mut self, mode: MarkupQuirksMode) {
		let mode = match mode {
			MarkupQuirksMode::Quirks => QuirksMode::Quirks,
			MarkupQuirksMode::LimitedQuirks => QuirksMode::LimitedQuirks,
			MarkupQuirksMode::NoQuirks => QuirksMode::NoQuirks,
		};
		self.document.set_quirks_mode(mode);
	}

	fn append_before_sibling(
		&mut self,
		sibling: &Self::Handle,
		new_node: NodeOrText<Self::Handle>,
	) {
		let parent = sibling.parent_node().unwrap();
		insert(&parent, Some(sibling.clone()), new_node)
	}

	fn add_attrs_if_missing(&mut self, target: &Self::Handle, attrs: Vec<html5ever::Attribute>) {
		let elem = target.downcast::<Element>();
		for attr in attrs {
			elem.set_attribute(attr.name, String::from(&attr.value), None);
		}
	}

	fn associate_with_form(
		&mut self,
		_target: &Self::Handle,
		_form: &Self::Handle,
		_nodes: (&Self::Handle, Option<&Self::Handle>),
	) {
		todo!()
	}

	fn remove_from_parent(&mut self, target: &Self::Handle) {
		if let Some(parent) = target.parent_node() {
			parent.remove_child(target.clone()).unwrap();
		}
	}

	fn reparent_children(&mut self, node: &Self::Handle, new_parent: &Self::Handle) {
		while let Some(child) = node.first_child() {
			new_parent.append_child(child).unwrap();
		}
	}

	fn is_mathml_annotation_xml_integration_point(&self, _handle: &Self::Handle) -> bool {
		false
	}

	fn set_current_line(&mut self, line_number: u64) {
		self.current_line = line_number;
	}

	fn complete_script(
		&mut self,
		_node: &Self::Handle,
	) -> html5ever::tree_builder::NextParserState {
		html5ever::tree_builder::NextParserState::Continue
	}
}

fn insert(parent: &Rc<Node>, reference_child: Option<Rc<Node>>, child: NodeOrText<Rc<Node>>) -> () {
	match child {
		NodeOrText::AppendNode(n) => {
			parent.insert_before(n, reference_child).unwrap();
		},
		NodeOrText::AppendText(t) => {
			// https://html.spec.whatwg.org/multipage/#insert-a-character
			let text = reference_child
				.clone()
				.and_then(|node| node.prev_sibling())
				.or_else(|| parent.last_child())
				.and_then(|node| Some(downcast::<Node, Text>(node)));

			if let Some(ref text) = text {
				text.upcast::<CharacterData>().append_data(&t);
			} else {
				let text = Text::create(String::from(t), parent.owner_doc().unwrap());
				parent.insert_before(upcast(text), reference_child).unwrap();
			}
		},
	}
}

/// https://html.spec.whatwg.org/multipage/#create-an-element-for-the-token
fn create_element_for_token(
	name: QualName,
	attrs: Vec<Attribute>,
	document: Rc<Document>,
) -> Rc<Element> {
	let is = attrs
		.iter()
		.find(|attr| attr.name.local.eq_str_ignore_ascii_case("is"))
		.map(|attr| LocalName::from(&*attr.value));

	let element = Element::create(name, is, document);

	for attr in attrs {
		element.set_attribute(attr.name, String::from(attr.value), None);
	}

	element
}
