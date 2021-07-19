use std::rc::Rc;

use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink};

use crate::{
    comment::Comment, document::Document, dom_object::Dom, element::Element, inheritance::Castable,
    node::Node, not_supported,
};

pub struct DomParser {
    document: Dom<Document>,
}

impl TreeSink for DomParser {
    type Output = Self;
    type Handle = Dom<Node>;

    fn finish(self) -> Self::Output {
        self
    }

    fn parse_error(&mut self, msg: std::borrow::Cow<'static, str>) {
        todo!()
    }

    fn get_document(&mut self) -> Self::Handle {
        Dom::from_ref(self.document.upcast())
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
        let element = create_element(name, attrs, &self.document);
        Dom::from_ref(element.upcast())
    }

    fn create_comment(&mut self, text: html5ever::tendril::StrTendril) -> Self::Handle {
        let comment = Comment::new(String::from(text), &self.document);
        Dom::from_ref(comment.upcast())
    }

    fn create_pi(
        &mut self,
        target: html5ever::tendril::StrTendril,
        data: html5ever::tendril::StrTendril,
    ) -> Self::Handle {
        not_supported!()
    }

    fn append(&mut self, parent: &Self::Handle, child: NodeOrText<Self::Handle>) {
        todo!()
    }

    fn append_based_on_parent_node(
        &mut self,
        element: &Self::Handle,
        prev_element: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    ) {
        todo!()
    }

    fn append_doctype_to_document(
        &mut self,
        name: html5ever::tendril::StrTendril,
        public_id: html5ever::tendril::StrTendril,
        system_id: html5ever::tendril::StrTendril,
    ) {
        todo!()
    }

    fn mark_script_already_started(&mut self, _node: &Self::Handle) {}

    fn pop(&mut self, _node: &Self::Handle) {}

    fn get_template_contents(&mut self, target: &Self::Handle) -> Self::Handle {
        todo!()
    }

    fn same_node(&self, x: &Self::Handle, y: &Self::Handle) -> bool {
        todo!()
    }

    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        self.document.set_quirks_mode(mode);
    }

    fn append_before_sibling(
        &mut self,
        sibling: &Self::Handle,
        new_node: NodeOrText<Self::Handle>,
    ) {
        todo!()
    }

    fn add_attrs_if_missing(&mut self, target: &Self::Handle, attrs: Vec<html5ever::Attribute>) {
        todo!()
    }

    fn associate_with_form(
        &mut self,
        _target: &Self::Handle,
        _form: &Self::Handle,
        _nodes: (&Self::Handle, Option<&Self::Handle>),
    ) {
    }

    fn remove_from_parent(&mut self, target: &Self::Handle) {
        todo!()
    }

    fn reparent_children(&mut self, node: &Self::Handle, new_parent: &Self::Handle) {
        todo!()
    }

    fn is_mathml_annotation_xml_integration_point(&self, _handle: &Self::Handle) -> bool {
        false
    }

    fn set_current_line(&mut self, _line_number: u64) {}

    fn complete_script(
        &mut self,
        _node: &Self::Handle,
    ) -> html5ever::tree_builder::NextParserState {
        html5ever::tree_builder::NextParserState::Continue
    }
}

fn create_element(
    name: html5ever::QualName,
    attrs: Vec<html5ever::Attribute>,
    document: &Dom<Document>,
) -> Element {
    todo!()
}
