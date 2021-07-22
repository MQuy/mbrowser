use std::rc::Rc;

use html5ever::{
    tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink},
    Attribute, LocalName, QualName,
};
use log::debug;

use crate::{
    comment::Comment, document::Document, documenttype::DocumentType, element::Element,
    inheritance::Castable, node::Node, not_supported, text::Text,
};

pub struct DomParser {
    document: Rc<Document>,
}

impl TreeSink for DomParser {
    type Output = Self;
    type Handle = Rc<Node>;

    fn finish(self) -> Self::Output {
        self
    }

    fn parse_error(&mut self, msg: std::borrow::Cow<'static, str>) {
        debug!("Parse error: {}", msg);
    }

    fn get_document(&mut self) -> Self::Handle {
        Rc::new(self.document.upcast().clone())
    }

    fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> html5ever::ExpandedName<'a> {
        let elem = target.downcast::<Element>();
        html5ever::ExpandedName {
            ns: elem.namespace(),
            local: elem.local_name(),
        }
    }

    fn create_element(
        &mut self,
        name: html5ever::QualName,
        attrs: Vec<html5ever::Attribute>,
        _flags: ElementFlags,
    ) -> Self::Handle {
        let element = create_element_for_token(name, attrs, &self.document);
        Rc::new(element.upcast().clone())
    }

    fn create_comment(&mut self, text: html5ever::tendril::StrTendril) -> Self::Handle {
        let comment = Comment::new(String::from(text), Rc::downgrade(&self.document));
        Rc::new(comment.upcast::<Node>().clone())
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
        if element.get_parent_node().is_some() {
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
        let doc = &*self.document;
        let doctype = DocumentType::new(
            String::from(name),
            String::from(public_id),
            String::from(system_id),
            Rc::downgrade(&self.document),
        );
        doc.upcast::<Node>()
            .append_child(doctype.upcast())
            .expect("Appending failed");
    }

    fn mark_script_already_started(&mut self, _node: &Self::Handle) {
        todo!()
    }

    fn pop(&mut self, _node: &Self::Handle) {
        todo!()
    }

    fn get_template_contents(&mut self, _target: &Self::Handle) -> Self::Handle {
        todo!()
    }

    fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
        x == y
    }

    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        self.document.set_quirks_mode(mode);
    }

    fn append_before_sibling(
        &mut self,
        sibling: &Self::Handle,
        new_node: NodeOrText<Self::Handle>,
    ) {
        let parent = sibling.get_parent_node().unwrap().upgrade().unwrap();
        insert(&parent, Some(sibling), new_node)
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
        if let Some(ref parent) = target.get_parent_node() {
            parent.upgrade().unwrap().remove_child(&*target).unwrap();
        }
    }

    fn reparent_children(&mut self, node: &Self::Handle, new_parent: &Self::Handle) {
        while let Some(ref child) = node.get_first_child() {
            new_parent.append_child(&child).unwrap();
        }
    }

    fn is_mathml_annotation_xml_integration_point(&self, _handle: &Self::Handle) -> bool {
        false
    }

    fn set_current_line(&mut self, _line_number: u64) {
        todo!()
    }

    fn complete_script(
        &mut self,
        _node: &Self::Handle,
    ) -> html5ever::tree_builder::NextParserState {
        html5ever::tree_builder::NextParserState::Continue
    }
}

fn insert(parent: &Rc<Node>, reference_child: Option<&Node>, child: NodeOrText<Rc<Node>>) -> () {
    match child {
        NodeOrText::AppendNode(n) => {
            parent.insert_before(&n, reference_child).unwrap();
        }
        NodeOrText::AppendText(t) => {
            let text = Text::new(String::from(t), parent.owner_doc());
            parent
                .insert_before(text.upcast(), reference_child)
                .unwrap();
        }
    }
}

/// https://html.spec.whatwg.org/multipage/#create-an-element-for-the-token
fn create_element_for_token(
    name: QualName,
    attrs: Vec<Attribute>,
    document: &Rc<Document>,
) -> Rc<Element> {
    let is = attrs
        .iter()
        .find(|attr| attr.name.local.eq_str_ignore_ascii_case("is"))
        .map(|attr| LocalName::from(&*attr.value));

    let element = Element::create(name, is, Rc::downgrade(document));

    for attr in attrs {
        element.set_attribute(attr.name, String::from(attr.value), None);
    }

    element
}
