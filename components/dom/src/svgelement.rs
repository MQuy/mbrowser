use crate::{
    element::Element,
    inheritance::{Castable, DerivedFrom},
    node::Node,
};

#[derive(Clone)]
pub struct SVGElement {
    element: Element,
}

impl Castable for SVGElement {}
impl DerivedFrom<Node> for SVGElement {}
impl DerivedFrom<Element> for SVGElement {}
