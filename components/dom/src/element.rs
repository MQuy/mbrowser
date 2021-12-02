use std::cell::{Cell, Ref, RefCell};
use std::rc::Rc;

use common::not_supported;
use css::element_state::ElementState;
use css::properties::declaration_block::{parse_style_attribute, PropertyDeclarationBlock};
use css::selectors::nonts_pseudo_class::NonTSPseudoClass;
use css::selectors::pseudo_element::PseudoElement;
use css::selectors::select::Selectors;
use css::stylesheets::css_rule::CssRuleType;
use css::values::{CSSString, Ident};
use html5ever::{local_name, namespace_url, ns, LocalName, Namespace, Prefix, QualName};
use selectors::attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint};
use selectors::context::MatchingContext;
use selectors::matching::ElementSelectorFlags;
use selectors::OpaqueElement;

use crate::attr::{Attr, AttrValue};
use crate::characterdata::CharacterData;
use crate::document::Document;
use crate::global_scope::{GlobalScope, NodeRef};
use crate::htmlbodyelement::HTMLBodyElement;
use crate::htmldivelement::HTMLDivElement;
use crate::htmlelement::HTMLElement;
use crate::htmlheadelement::HTMLHeadElement;
use crate::htmlheadingelement::{HTMLHeadingElement, HeadingLevel};
use crate::htmlhtmlelement::HTMLHtmlElement;
use crate::htmlparagraphelement::HTMLParagraphElement;
use crate::htmlspanelement::HTMLSpanElement;
use crate::htmlunknownelement::HTMLUnknownElement;
use crate::inheritance::{downcast, upcast, Castable};
use crate::node::Node;
use crate::nodetype::{ElementTypeId, HTMLElementTypeId, NodeTypeId};
use crate::svgelement::SVGElement;
use crate::svgsvgelement::SVGSVGElement;
use crate::virtualmethods::{vtable_for, VirtualMethods};

#[derive(Clone, Debug)]
pub struct Element {
	node: Node,
	prefix: Option<Prefix>,
	local_name: LocalName,
	namespace: Namespace,
	is: RefCell<Option<LocalName>>,
	attrs: RefCell<Vec<Rc<Attr>>>,
	state: Cell<ElementState>,
	style_attribute: RefCell<Option<PropertyDeclarationBlock>>,
	selector_flags: RefCell<ElementSelectorFlags>,
}

impl crate::inheritance::Castable for Element {}
impl crate::inheritance::DerivedFrom<Node> for Element {}

impl Element {
	pub fn new(local_name: LocalName, namespace: Namespace, prefix: Option<Prefix>, document: Rc<Document>) -> Self {
		Element::new_inherited(
			NodeTypeId::Element(ElementTypeId::Element),
			local_name,
			namespace,
			prefix,
			document,
		)
	}

	pub fn new_inherited(
		node_type_id: NodeTypeId,
		local_name: LocalName,
		namespace: Namespace,
		prefix: Option<Prefix>,
		document: Rc<Document>,
	) -> Self {
		Self {
			node: Node::new(node_type_id, Some(document)),
			local_name,
			namespace,
			prefix,
			is: RefCell::new(None),
			style_attribute: RefCell::new(None),
			attrs: RefCell::new(Vec::new()),
			state: Cell::new(ElementState::empty()),
			selector_flags: RefCell::new(ElementSelectorFlags::empty()),
		}
	}

	pub fn namespace(&self) -> &Namespace {
		&self.namespace
	}

	pub fn local_name(&self) -> &LocalName {
		&self.local_name
	}

	pub fn set_attribute(&self, qname: QualName, value: String, prefix: Option<Prefix>) {
		if let Some(attr) = self
			.attrs
			.borrow()
			.iter()
			.find(|attr| *attr.local_name() == qname.local && *attr.namespace() == qname.ns)
		{
			attr.set_value(AttrValue::String(value));
			return;
		}

		let name = match prefix {
			None => qname.local.clone(),
			Some(ref prefix) => {
				let name = format!("{}:{}", prefix, qname.local);
				LocalName::from(name)
			},
		};
		let value = self.parse_attribute(&qname.ns, &qname.local, value);
		self.push_new_attribute(qname.local, value, name, qname.ns, prefix);
	}

	pub fn push_new_attribute(
		&self,
		local_name: LocalName,
		value: AttrValue,
		name: LocalName,
		namespace: Namespace,
		prefix: Option<Prefix>,
	) {
		let attr = Attr::new(
			local_name,
			value,
			name,
			namespace,
			prefix,
			downcast(GlobalScope::get_node(self.node.id())),
		);
		self.push_attribute(attr);
	}

	pub fn push_attribute(&self, attr: Attr) {
		if attr.namespace() == &ns!() {
			vtable_for(self.upcast()).attribute_mutated(&attr, AttributeMutation::Set(None));
		}
		self.attrs.borrow_mut().push(Rc::from(attr));
	}

	pub fn get_attribute(&self, namespace: &Namespace, local_name: &LocalName) -> Option<Rc<Attr>> {
		self.attrs
			.borrow()
			.iter()
			.find(|attr| *attr.local_name() == *local_name && *attr.namespace() == *namespace)
			.map(|attr| attr.clone())
	}

	pub fn attrs(&self) -> RefCell<Vec<Rc<Attr>>> {
		self.attrs.clone()
	}

	pub fn parse_attribute(&self, namespace: &Namespace, local_name: &LocalName, value: String) -> AttrValue {
		if *namespace == ns!() {
			vtable_for(self.upcast()).parse_plain_attribute(local_name, value)
		} else {
			AttrValue::String(value.into())
		}
	}

	pub fn has_attribute(&self, local_name: &LocalName) -> bool {
		self.attrs
			.borrow()
			.iter()
			.any(|attr| attr.local_name() == local_name && attr.namespace() == &ns!())
	}

	pub fn set_is(&self, is: LocalName) {
		*self.is.borrow_mut() = Some(is);
	}

	pub fn create(name: QualName, is: Option<LocalName>, document: Rc<Document>) -> Rc<Element> {
		let prefix = name.prefix.clone();
		let element = match name.ns {
			ns!(html) => create_html_element(name, prefix, is, document),
			ns!(svg) => create_svg_element(name, prefix, document),
			_ => Rc::new(Element::new(name.local, name.ns, prefix, document)),
		};
		GlobalScope::add_node(upcast(element.clone()));
		element
	}

	pub fn state(&self) -> ElementState {
		self.state.get()
	}

	#[inline]
	pub fn insert_selector_flags(&self, flags: ElementSelectorFlags) {
		let f = self.selector_flags.borrow();
		self.selector_flags.borrow_mut().set(*f | flags, true);
	}

	#[inline]
	pub fn has_selector_flags(&self, flags: ElementSelectorFlags) -> bool {
		self.selector_flags.borrow().contains(flags)
	}

	pub fn style_attribute(&self) -> &RefCell<Option<PropertyDeclarationBlock>> {
		&self.style_attribute
	}
}

impl VirtualMethods for Element {
	fn super_type(&self) -> Option<&dyn VirtualMethods> {
		Some(self.upcast::<Node>() as &dyn VirtualMethods)
	}

	fn parse_plain_attribute(&self, name: &LocalName, value: String) -> AttrValue {
		match name {
			&local_name!("id") => AttrValue::String(value),
			&local_name!("name") => AttrValue::String(value),
			&local_name!("class") => AttrValue::from_serialized_tokenlist(value),
			_ => self.super_type().unwrap().parse_plain_attribute(name, value),
		}
	}

	fn attribute_mutated(&self, attr: &Attr, mutation: AttributeMutation) {
		match attr.local_name() {
			&local_name!("style") => {
				let changed_style = match mutation {
					AttributeMutation::Set(..) => match self.node.owner_doc() {
						Some(document) => Some(parse_style_attribute(
							&attr.value(),
							Some(document.window().unwrap().error_reporter()),
							document.quirks_mode(),
							CssRuleType::Style,
						)),
						None => not_supported!(),
					},
					AttributeMutation::Removed => None,
				};
				*self.style_attribute.borrow_mut() = changed_style;
			},
			_ => {},
		}
	}
}

macro_rules! make_element(
        ($ctor: ident, $local: expr, $prefix: ident, $document: ident) => ({
            let obj = $ctor::new($local, $prefix, $document);
            Rc::new(obj.upcast::<Element>().clone())
        });
        ($ctor: ident, $local: expr, $prefix: ident, $document: ident, $($arg: expr),+) => ({
            let obj = $ctor::new($local, $prefix, $document, $($arg),+);
            Rc::new(obj.upcast::<Element>().clone())
        })
    );

fn create_svg_element(name: QualName, prefix: Option<Prefix>, document: Rc<Document>) -> Rc<Element> {
	assert_eq!(name.ns, ns!(svg));

	match name.local {
		local_name!("svg") => make_element!(SVGSVGElement, name.local, prefix, document),
		_ => make_element!(SVGElement, name.local, prefix, document),
	}
}

// https://dom.spec.whatwg.org/#concept-create-element
fn create_html_element(
	name: QualName,
	prefix: Option<Prefix>,
	is: Option<LocalName>,
	document: Rc<Document>,
) -> Rc<Element> {
	assert_eq!(name.ns, ns!(html));

	let result = create_native_html_element(name, prefix, document);

	if let Some(is) = is {
		result.set_is(is);
	}

	result
}

fn create_native_html_element(name: QualName, prefix: Option<Prefix>, document: Rc<Document>) -> Rc<Element> {
	assert_eq!(name.ns, ns!(html));

	// This is a big match, and the IDs for inline-interned atoms are not very structured.
	// Perhaps we should build a perfect hash from those IDs instead.
	// https://html.spec.whatwg.org/multipage/#elements-in-the-dom
	match name.local {
		local_name!("b") => make_element!(HTMLElement, name.local, prefix, document),
		local_name!("body") => make_element!(HTMLBodyElement, name.local, prefix, document),
		local_name!("div") => make_element!(HTMLDivElement, name.local, prefix, document),
		local_name!("footer") => make_element!(HTMLElement, name.local, prefix, document),
		local_name!("h1") => make_element!(HTMLHeadingElement, name.local, prefix, document, HeadingLevel::Heading1),
		local_name!("h2") => make_element!(HTMLHeadingElement, name.local, prefix, document, HeadingLevel::Heading2),
		local_name!("h3") => make_element!(HTMLHeadingElement, name.local, prefix, document, HeadingLevel::Heading3),
		local_name!("h4") => make_element!(HTMLHeadingElement, name.local, prefix, document, HeadingLevel::Heading4),
		local_name!("h5") => make_element!(HTMLHeadingElement, name.local, prefix, document, HeadingLevel::Heading5),
		local_name!("h6") => make_element!(HTMLHeadingElement, name.local, prefix, document, HeadingLevel::Heading6),
		local_name!("head") => make_element!(HTMLHeadElement, name.local, prefix, document),
		local_name!("header") => make_element!(HTMLElement, name.local, prefix, document),
		local_name!("html") => make_element!(HTMLHtmlElement, name.local, prefix, document),
		local_name!("span") => make_element!(HTMLSpanElement, name.local, prefix, document),
		local_name!("strong") => make_element!(HTMLElement, name.local, prefix, document),
		local_name!("p") => make_element!(HTMLParagraphElement, name.local, prefix, document),
		_ if is_valid_custom_element_name(&*name.local) => {
			make_element!(HTMLElement, name.local, prefix, document)
		},
		_ => make_element!(HTMLUnknownElement, name.local, prefix, document),
	}
}

/// <https://html.spec.whatwg.org/multipage/#valid-custom-element-name>
pub fn is_valid_custom_element_name(name: &str) -> bool {
	// Custom elment names must match:
	// PotentialCustomElementName ::= [a-z] (PCENChar)* '-' (PCENChar)*

	let mut chars = name.chars();
	if !chars.next().map_or(false, |c| c >= 'a' && c <= 'z') {
		return false;
	}

	let mut has_dash = false;

	for c in chars {
		if c == '-' {
			has_dash = true;
			continue;
		}

		if !is_potential_custom_element_char(c) {
			return false;
		}
	}

	if !has_dash {
		return false;
	}

	if name == "annotation-xml"
		|| name == "color-profile"
		|| name == "font-face"
		|| name == "font-face-src"
		|| name == "font-face-uri"
		|| name == "font-face-format"
		|| name == "font-face-name"
		|| name == "missing-glyph"
	{
		return false;
	}

	true
}

/// Check if this character is a PCENChar
/// <https://html.spec.whatwg.org/multipage/#prod-pcenchar>
fn is_potential_custom_element_char(c: char) -> bool {
	c == '-'
		|| c == '.'
		|| c == '_'
		|| c == '\u{B7}'
		|| (c >= '0' && c <= '9')
		|| (c >= 'a' && c <= 'z')
		|| (c >= '\u{C0}' && c <= '\u{D6}')
		|| (c >= '\u{D8}' && c <= '\u{F6}')
		|| (c >= '\u{F8}' && c <= '\u{37D}')
		|| (c >= '\u{37F}' && c <= '\u{1FFF}')
		|| (c >= '\u{200C}' && c <= '\u{200D}')
		|| (c >= '\u{203F}' && c <= '\u{2040}')
		|| (c >= '\u{2070}' && c <= '\u{2FEF}')
		|| (c >= '\u{3001}' && c <= '\u{D7FF}')
		|| (c >= '\u{F900}' && c <= '\u{FDCF}')
		|| (c >= '\u{FDF0}' && c <= '\u{FFFD}')
		|| (c >= '\u{10000}' && c <= '\u{EFFFF}')
}

#[derive(Clone, Copy)]
pub enum AttributeMutation<'a> {
	/// The attribute is set, keep track of old value.
	/// <https://dom.spec.whatwg.org/#attribute-is-set>
	Set(Option<&'a AttrValue>),

	/// The attribute is removed.
	/// <https://dom.spec.whatwg.org/#attribute-is-removed>
	Removed,
}

impl<'a> AttributeMutation<'a> {
	pub fn is_removal(&self) -> bool {
		match *self {
			AttributeMutation::Removed => true,
			AttributeMutation::Set(..) => false,
		}
	}

	pub fn new_value<'b>(&self, attr: &'b Attr) -> Option<Ref<'b, AttrValue>> {
		match *self {
			AttributeMutation::Set(_) => Some(attr.value()),
			AttributeMutation::Removed => None,
		}
	}
}

impl selectors::Element for NodeRef {
	type Impl = Selectors;

	fn opaque(&self) -> OpaqueElement {
		OpaqueElement::new(&self)
	}

	fn parent_element(&self) -> Option<Self> {
		self.parent()
	}

	fn parent_node_is_shadow_root(&self) -> bool {
		false
	}

	fn containing_shadow_host(&self) -> Option<Self> {
		None
	}

	fn is_pseudo_element(&self) -> bool {
		false
	}

	fn prev_sibling_element(&self) -> Option<Self> {
		self.prev_sibling()
	}

	fn next_sibling_element(&self) -> Option<Self> {
		self.next_sibling()
	}

	fn is_html_element_in_html_document(&self) -> bool {
		self.namespace() == ns!(html)
	}

	fn has_local_name(&self, local_name: &LocalName) -> bool {
		self.local_name() == *local_name
	}

	fn has_namespace(&self, ns: &Namespace) -> bool {
		self.namespace() == *ns
	}

	fn is_same_type(&self, other: &Self) -> bool {
		self.local_name() == other.local_name() && self.namespace() == other.namespace()
	}

	fn attr_matches(
		&self,
		ns: &NamespaceConstraint<&css::Namespace>,
		local_name: &css::LocalName,
		operation: &AttrSelectorOperation<&CSSString>,
	) -> bool {
		match *ns {
			NamespaceConstraint::Specific(ref ns) => self
				.get_attribute(&ns.0, &local_name.0)
				.map_or(false, |attr| attr.value().eval_selector(operation)),
			NamespaceConstraint::Any => self
				.attrs()
				.borrow()
				.iter()
				.any(|attr| *attr.local_name() == *local_name.0 && attr.value().eval_selector(operation)),
		}
	}

	fn match_non_ts_pseudo_class<F>(
		&self,
		pseudo_class: &NonTSPseudoClass,
		context: &mut MatchingContext<Self::Impl>,
		_flags_setter: &mut F,
	) -> bool
	where
		F: FnMut(&Self, ElementSelectorFlags),
	{
		match *pseudo_class {
			NonTSPseudoClass::Autofill
			| NonTSPseudoClass::Defined
			| NonTSPseudoClass::Focus
			| NonTSPseudoClass::Enabled
			| NonTSPseudoClass::Disabled
			| NonTSPseudoClass::Checked
			| NonTSPseudoClass::Fullscreen
			| NonTSPseudoClass::Indeterminate
			| NonTSPseudoClass::PlaceholderShown
			| NonTSPseudoClass::Target
			| NonTSPseudoClass::Valid
			| NonTSPseudoClass::Invalid
			| NonTSPseudoClass::Required
			| NonTSPseudoClass::Optional
			| NonTSPseudoClass::ReadWrite
			| NonTSPseudoClass::FocusWithin
			| NonTSPseudoClass::FocusVisible
			| NonTSPseudoClass::InRange
			| NonTSPseudoClass::OutOfRange
			| NonTSPseudoClass::Default
			| NonTSPseudoClass::Active
			| NonTSPseudoClass::Hover => self.state().contains(pseudo_class.state_flag()),

			NonTSPseudoClass::ReadOnly => self.state().contains(pseudo_class.state_flag()),

			NonTSPseudoClass::AnyLink => self.is_link(),
			NonTSPseudoClass::Link => self.is_link() && context.visited_handling().matches_unvisited(),
			NonTSPseudoClass::Visited => self.is_link() && context.visited_handling().matches_visited(),
		}
	}

	fn match_pseudo_element(
		&self,
		_pseudo_element: &PseudoElement,
		_context: &mut MatchingContext<Self::Impl>,
	) -> bool {
		false
	}

	fn is_link(&self) -> bool {
		match self.node_type_id() {
			// https://html.spec.whatwg.org/multipage/#selector-link
			NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLAnchorElement))
			| NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLAreaElement))
			| NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLLinkElement)) => {
				self.has_attribute(&local_name!("href"))
			},
			_ => false,
		}
	}

	fn is_html_slot_element(&self) -> bool {
		false
	}

	fn has_id(&self, id: &Ident, case_sensitivity: CaseSensitivity) -> bool {
		self.get_attribute(&ns!(), &local_name!("id"))
			.as_ref()
			.map_or(false, |attr| {
				case_sensitivity.eq(id.0.as_bytes(), attr.value().to_string().as_bytes())
			})
	}

	fn has_class(&self, name: &Ident, case_sensitivity: CaseSensitivity) -> bool {
		self.get_attribute(&ns!(), &local_name!("class")).map_or(false, |attr| {
			attr.as_tokens().map_or(false, |values| {
				values
					.iter()
					.any(|class_name| case_sensitivity.eq(name.0.as_bytes(), class_name.as_bytes()))
			})
		})
	}

	fn imported_part(&self, _name: &Ident) -> Option<Ident> {
		None
	}

	fn is_part(&self, _name: &Ident) -> bool {
		false
	}

	fn is_empty(&self) -> bool {
		self.children().all(|node| {
			if node.node_type_id().is_element() {
				return false;
			}
			if node.node_type_id().is_character_data_text() {
				return node.downcast::<CharacterData>().data().is_empty();
			}
			return true;
		})
	}

	fn is_root(&self) -> bool {
		match self.parent_node() {
			None => false,
			Some(node) => node.node_type_id().is_document(),
		}
	}
}
