use html5ever::LocalName;

/// <https://html.spec.whatwg.org/multipage/#custom-element-definition>
pub struct CustomElementDefinition {
    pub name: LocalName,
    pub local_name: LocalName,
    pub observed_attributes: Vec<String>,
}
