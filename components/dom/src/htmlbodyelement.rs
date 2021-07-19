use crate::{
    element::Element,
    htmlelement::HTMLElement,
    inheritance::{Castable, DerivedFrom},
    node::Node,
};

#[derive(Clone)]
pub struct HTMLBodyElement {
    html_element: HTMLElement,
}

impl Castable for HTMLBodyElement {}
impl DerivedFrom<Node> for HTMLBodyElement {}
impl DerivedFrom<Element> for HTMLBodyElement {}
impl DerivedFrom<HTMLElement> for HTMLBodyElement {}
