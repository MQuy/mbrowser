use std::rc::Weak;

use html5ever::{local_name, namespace_url};
use html5ever::{ns, LocalName, Prefix};

use crate::attr::AttrValue;
use crate::virtualmethods::VirtualMethods;
use crate::{
    document::Document,
    element::Element,
    inheritance::{Castable, DerivedFrom},
    node::Node,
    nodetype::{ElementTypeId, NodeTypeId},
};

#[derive(Clone)]
pub struct HTMLElement {
    element: Element,
}

impl Castable for HTMLElement {}
impl DerivedFrom<Node> for HTMLElement {}
impl DerivedFrom<Element> for HTMLElement {}

impl HTMLElement {
    pub fn new(local_name: LocalName, prefix: Option<Prefix>, document: Weak<Document>) -> Self {
        HTMLElement::new_inherited(
            NodeTypeId::Element(ElementTypeId::Element),
            local_name,
            prefix,
            document,
        )
    }

    pub fn new_inherited(
        node_type_id: NodeTypeId,
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: Weak<Document>,
    ) -> Self {
        HTMLElement {
            element: Element::new_inherited(node_type_id, local_name, ns!(html), prefix, document),
        }
    }
}

impl VirtualMethods for HTMLElement {
    fn super_type(&self) -> Option<&dyn VirtualMethods> {
        Some(self.upcast::<Element>() as &dyn VirtualMethods)
    }

    fn parse_plain_attribute(&self, name: &LocalName, value: String) -> AttrValue {
        match name {
            &local_name!("itemprop") => AttrValue::from_serialized_tokenlist(value.into()),
            &local_name!("itemtype") => AttrValue::from_serialized_tokenlist(value.into()),
            _ => self
                .super_type()
                .unwrap()
                .parse_plain_attribute(name, value),
        }
    }
}
