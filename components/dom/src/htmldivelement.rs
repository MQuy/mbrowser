use crate::{
    element::Element,
    htmlelement::HTMLElement,
    inheritance::{Castable, DerivedFrom},
    node::Node,
};

#[derive(Clone)]
pub struct HTMLDivElement {
    html_element: HTMLElement,
}

impl Castable for HTMLDivElement {}
impl DerivedFrom<Node> for HTMLDivElement {}
impl DerivedFrom<Element> for HTMLDivElement {}
impl DerivedFrom<HTMLElement> for HTMLDivElement {}
