use cssparser::SourceLocation;
use selectors::SelectorList;

use crate::declaration_block::DeclarationBlock;
use crate::selectors::select_impl::SelectorImpl;

/// A style rule, with selectors and declarations.
#[derive(Clone)]
pub struct StyleRule {
    /// The list of selectors in this rule.
    pub selectors: SelectorList<SelectorImpl>,
    /// The declaration block with the properties it contains.
    pub block: DeclarationBlock,
    /// The location in the sheet where it was found.
    pub source_location: SourceLocation,
}
