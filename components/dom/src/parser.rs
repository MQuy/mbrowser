use html5ever::tree_builder::{ElementFlags, NextParserState, NodeOrText, QuirksMode, TreeSink};

use crate::node::Node;

pub struct HTMLParser {}

impl TreeSink for HTMLParser {
    type Output = Self;
    type Handle = Node;

    fn finish(self) -> Self::Output {
        todo!()
    }

    fn parse_error(&mut self, msg: std::borrow::Cow<'static, str>) {
        todo!()
    }

    fn get_document(&mut self) -> Self::Handle {
        todo!()
    }

    fn elem_name<'a>(&'a self, target: &'a Self::Handle) -> html5ever::ExpandedName<'a> {
        todo!()
    }

    fn create_element(
        &mut self,
        name: html5ever::QualName,
        attrs: Vec<html5ever::Attribute>,
        flags: ElementFlags,
    ) -> Self::Handle {
        todo!()
    }

    fn create_comment(&mut self, text: html5ever::tendril::StrTendril) -> Self::Handle {
        todo!()
    }

    fn create_pi(
        &mut self,
        target: html5ever::tendril::StrTendril,
        data: html5ever::tendril::StrTendril,
    ) -> Self::Handle {
        todo!()
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
        todo!()
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

    fn complete_script(&mut self, _node: &Self::Handle) -> NextParserState {
        NextParserState::Continue
    }
}
