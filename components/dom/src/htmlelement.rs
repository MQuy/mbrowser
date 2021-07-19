use crate::{
    element::Element,
    inheritance::{Castable, DerivedFrom},
    node::Node,
};

#[derive(Clone)]
pub struct HTMLElement {
    element: Element,
}

impl Castable for HTMLElement {}
impl DerivedFrom<Node> for HTMLElement {}
impl DerivedFrom<Element> for HTMLElement {}
