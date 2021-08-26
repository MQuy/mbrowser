use std::cell::{Ref, RefCell};
use std::rc::Rc;

use html5ever::{local_name, namespace_url, ns, LocalName, Namespace, Prefix, QualName};

use crate::attr::{Attr, AttrValue};
use crate::document::Document;
use crate::htmlbodyelement::HTMLBodyElement;
use crate::htmldivelement::HTMLDivElement;
use crate::htmlelement::HTMLElement;
use crate::htmlheadelement::HTMLHeadElement;
use crate::htmlheadingelement::{HTMLHeadingElement, HeadingLevel};
use crate::htmlhtmlelement::HTMLHtmlElement;
use crate::htmlspanelement::HTMLSpanElement;
use crate::htmlunknownelement::HTMLUnknownElement;
use crate::inheritance::Castable;
use crate::node::Node;
use crate::nodetype::{ElementTypeId, NodeTypeId};
use crate::svgelement::SVGElement;
use crate::svgsvgelement::SVGSVGElement;
use crate::virtualmethods::{vtable_for, VirtualMethods};

#[derive(Clone)]
#[repr(C)]
pub struct Element {
	node: Node,
	prefix: Option<Prefix>,
	local_name: LocalName,
	namespace: Namespace,
	is: RefCell<Option<LocalName>>,
	attrs: RefCell<Vec<Rc<Attr>>>,
}

impl crate::inheritance::Castable for Element {}
impl crate::inheritance::DerivedFrom<Node> for Element {}

impl Element {
	pub fn new(
		local_name: LocalName,
		namespace: Namespace,
		prefix: Option<Prefix>,
		document: Rc<Document>,
	) -> Self {
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
			attrs: RefCell::new(Vec::new()),
		}
	}

	pub fn namespace(&self) -> &Namespace {
		&self.namespace
	}

	pub fn local_name(&self) -> &LocalName {
		&self.local_name
	}

	pub fn set_attribute(&self, qname: QualName, value: String, prefix: Option<Prefix>) {
		if let Some(attr) =
			self.attrs.borrow().iter().find(|attr| {
				*attr.get_local_name() == qname.local && *attr.get_namespace() == qname.ns
			}) {
			attr.set_value(AttrValue::String(value));
			return;
		}

		let name = match prefix {
			None => qname.local.clone(),
			Some(ref prefix) => {
				let name = format!("{}:{}", &**prefix, &*qname.local);
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
			Rc::new(self.clone()),
		);
		self.push_attribute(attr);
	}

	pub fn push_attribute(&self, attr: Attr) {
		self.attrs.borrow_mut().push(Rc::from(attr));
	}

	pub fn get_attribute(&self, namespace: &Namespace, local_name: &LocalName) -> Option<Rc<Attr>> {
		self.attrs
			.borrow()
			.iter()
			.find(|attr| {
				*attr.get_local_name() == *local_name && *attr.get_namespace() == *namespace
			})
			.map(|attr| attr.clone())
	}

	pub fn get_attrs(&self) -> Ref<'_, Vec<Rc<Attr>>> {
		self.attrs.borrow()
	}

	pub fn parse_attribute(
		&self,
		namespace: &Namespace,
		local_name: &LocalName,
		value: String,
	) -> AttrValue {
		if *namespace == ns!() {
			vtable_for(self.upcast()).parse_plain_attribute(local_name, value)
		} else {
			AttrValue::String(value.into())
		}
	}

	pub fn set_is(&self, is: LocalName) {
		*self.is.borrow_mut() = Some(is);
	}

	pub fn create(name: QualName, is: Option<LocalName>, document: Rc<Document>) -> Rc<Element> {
		let prefix = name.prefix.clone();
		match name.ns {
			ns!(html) => create_html_element(name, prefix, is, document),
			ns!(svg) => create_svg_element(name, prefix, document),
			_ => Rc::new(Element::new(name.local, name.ns, prefix, document)),
		}
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
			_ => self
				.super_type()
				.unwrap()
				.parse_plain_attribute(name, value),
		}
	}
}

fn create_svg_element(
	name: QualName,
	prefix: Option<Prefix>,
	document: Rc<Document>,
) -> Rc<Element> {
	assert_eq!(name.ns, ns!(svg));

	macro_rules! make(
        ($ctor:ident) => ({
            let obj = $ctor::new(name.local, prefix, document);
            Rc::new(obj.upcast::<Element>().clone())
        });
        ($ctor:ident, $($arg:expr),+) => ({
            let obj = $ctor::new(name.local, prefix, document, $($arg),+);
            Rc::new(obj.upcast::<Element>().clone())
        })
    );

	match name.local {
		local_name!("svg") => make!(SVGSVGElement),
		_ => make!(SVGElement),
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

fn create_native_html_element(
	name: QualName,
	prefix: Option<Prefix>,
	document: Rc<Document>,
) -> Rc<Element> {
	assert_eq!(name.ns, ns!(html));

	macro_rules! make(
        ($ctor:ident) => ({
            let obj = $ctor::new(name.local, prefix, document);
            Rc::new(obj.upcast::<Element>().clone())
        });
        ($ctor:ident, $($arg:expr),+) => ({
            let obj = $ctor::new(name.local, prefix, document, $($arg),+);
            Rc::new(obj.upcast::<Element>().clone())
        })
    );

	// This is a big match, and the IDs for inline-interned atoms are not very structured.
	// Perhaps we should build a perfect hash from those IDs instead.
	// https://html.spec.whatwg.org/multipage/#elements-in-the-dom
	match name.local {
		local_name!("b") => make!(HTMLElement),
		local_name!("body") => make!(HTMLBodyElement),
		local_name!("div") => make!(HTMLDivElement),
		local_name!("footer") => make!(HTMLElement),
		local_name!("h1") => make!(HTMLHeadingElement, HeadingLevel::Heading1),
		local_name!("h2") => make!(HTMLHeadingElement, HeadingLevel::Heading2),
		local_name!("h3") => make!(HTMLHeadingElement, HeadingLevel::Heading3),
		local_name!("h4") => make!(HTMLHeadingElement, HeadingLevel::Heading4),
		local_name!("h5") => make!(HTMLHeadingElement, HeadingLevel::Heading5),
		local_name!("h6") => make!(HTMLHeadingElement, HeadingLevel::Heading6),
		local_name!("head") => make!(HTMLHeadElement),
		local_name!("header") => make!(HTMLElement),
		local_name!("html") => make!(HTMLHtmlElement),
		local_name!("span") => make!(HTMLSpanElement),
		local_name!("strong") => make!(HTMLElement),
		_ if is_valid_custom_element_name(&*name.local) => make!(HTMLElement),
		_ => make!(HTMLUnknownElement),
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
