use std::vec::Drain;

use cssparser::{
    parse_important, AtRuleParser, CowRcStr, DeclarationListParser, DeclarationParser, Delimiter,
    ParseError, Parser,
};
use selectors::SelectorList;

use crate::declaration::{Declaration, Importance};
use crate::property_id::PropertyId;
use crate::selectors::select_impl::SelectorImpl;
use crate::stylesheets::rule_parser::StyleParseErrorKind;
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

    /// Returns whether the property is definitely new for this declaration
    /// block. It returns true when the declaration is a non-custom longhand
    /// and it doesn't exist in the block, and returns false otherwise.
    #[inline]
    fn is_definitely_new(&self, decl: &Declaration) -> bool {
        decl.id()
            .as_longhand()
            .map_or(false, |id| !self.longhands.contains(id))
    }

    /// Adds or overrides the declaration for a given property in this block.
    ///
    /// Returns whether the declaration has changed.
    ///
    /// This is only used for parsing and internal use.
    pub fn push(&mut self, declaration: Declaration, importance: Importance) -> bool {
        if !self.is_definitely_new(&declaration) {
            let mut index_to_remove = None;
            for (i, slot) in self.declarations.iter_mut().enumerate() {
                if slot.id() != declaration.id() {
                    continue;
                }

                let important = self.declarations_importance[i];

                // For declarations from parsing, non-important declarations
                // shouldn't override existing important one.
                if important && !importance.important() {
                    return false;
                }

                index_to_remove = Some(i);
                break;
            }

            if let Some(index) = index_to_remove {
                self.declarations.remove(index);
                self.declarations_importance.remove(index);
                self.declarations.push(declaration);
                self.declarations_importance.push(importance.important());
                return true;
            }
        }

        if let PropertyDeclarationId::Longhand(id) = declaration.id() {
            self.longhands.insert(id);
        }
        self.declarations.push(declaration);
        self.declarations_importance.push(importance.important());
        true
    }

    /// Adds or overrides the declaration for a given property in this block.
    ///
    /// See the documentation of `push` to see what impact `source` has when the
    /// property is already there.
    pub fn extend(&mut self, mut drain: Drain<Declaration>, importance: Importance) -> bool {
        let push_calls_count = drain.len();

        // With deduplication the actual length increase may be less than this.
        self.declarations.reserve(push_calls_count);

        let mut changed = false;
        for decl in &mut drain {
            changed |= self.push(decl, importance);
        }
        drain.fold(changed, |changed, decl| {
            changed | self.push(decl, importance)
        })
    }

    pub fn drain(&mut self) -> Drain<Declaration> {
        self.declarations.drain(..)
    }

    /// Reset to initial state
    pub fn clear(&mut self) {
        self.declarations.clear();
    }
}

/// A struct to parse property declarations.
struct PropertyDeclarationParser<'a, 'b: 'a> {
    context: &'a ParserContext<'b>,
    declarations: DeclarationBlock,
    /// The last parsed property id if any.
    last_parsed_property_id: Option<PropertyId>,
}

/// Default methods reject all at rules.
impl<'a, 'b, 'i> AtRuleParser<'i> for PropertyDeclarationParser<'a, 'b> {
    type PreludeNoBlock = ();
    type PreludeBlock = ();
    type AtRule = Importance;
    type Error = StyleParseErrorKind<'i>;
}

impl<'a, 'b, 'i> DeclarationParser<'i> for PropertyDeclarationParser<'a, 'b> {
    type Declaration = Importance;
    type Error = StyleParseErrorKind<'i>;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Importance, ParseError<'i, Self::Error>> {
        let id = match PropertyId::parse(&name, self.context) {
            Ok(id) => id,
            Err(..) => {
                return Err(input.new_custom_error(StyleParseErrorKind::UnknownProperty(name)));
            },
        };
        if self.context.error_reporting_enabled() {
            self.last_parsed_property_id = Some(id.clone());
        }
        input.parse_until_before(Delimiter::Bang, |input| {
            Declaration::parse_into(self.declarations, id, self.context, input)
        })?;
        let importance = match input.try_parse(parse_important) {
            Ok(()) => Importance::Important,
            Err(_) => Importance::Normal,
        };
        // In case there is still unparsed text in the declaration, we should roll back.
        input.expect_exhausted()?;
        Ok(importance)
    }
}

/// Parse a list of property declarations and return a property declaration
/// block.
pub fn parse_property_declaration_list(
    context: &ParserContext,
    input: &mut Parser,
    selectors: Option<&SelectorList<SelectorImpl>>,
) -> DeclarationBlock {
    let mut declarations = DeclarationBlock::new();
    let mut block = DeclarationBlock::new();
    let parser = PropertyDeclarationParser {
        context,
        last_parsed_property_id: None,
        declarations: DeclarationBlock::new(),
    };
    let mut iter = DeclarationListParser::new(input, parser);
    let mut errors = Vec::new();
    while let Some(declaration) = iter.next() {
        match declaration {
            Ok(importance) => {
                block.extend(iter.parser.declarations.drain(), importance);
                // We've successfully parsed a declaration, so forget about
                // `last_parsed_property_id`. It'd be wrong to associate any
                // following error with this property.
                iter.parser.last_parsed_property_id = None;
            },
            Err((error, slice)) => {
                iter.parser.declarations.clear();

                if context.error_reporting_enabled() {
                    let property = iter.parser.last_parsed_property_id.take();
                    errors.push((error, slice, property));
                }
            },
        }
    }

    if !errors.is_empty() {
        todo!()
    }

    block
}
