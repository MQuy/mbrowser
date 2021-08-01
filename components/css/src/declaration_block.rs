use cssparser::Parser;
use selectors::SelectorList;

use crate::declaration::Declaration;
use crate::selectors::select_impl::SelectorImpl;
use crate::stylesheets::stylesheet::ParserContext;

#[derive(Clone)]
pub struct DeclarationBlock {
    declarations: Vec<Declaration>,
}

impl DeclarationBlock {
    #[inline]
    pub fn new() -> Self {
        DeclarationBlock {
            declarations: Vec::new(),
        }
    }
}

/// Parse a list of property declarations and return a property declaration
/// block.
pub fn parse_property_declaration_list(
    context: &ParserContext,
    input: &mut Parser,
    selectors: Option<&SelectorList<SelectorImpl>>,
) -> DeclarationBlock {
    todo!()
}
