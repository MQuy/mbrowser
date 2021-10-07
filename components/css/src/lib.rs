#[macro_use]
extern crate bitflags;

pub mod computed_values;
pub mod css_writer;
pub mod element_state;
pub mod error_reporting;
pub mod media_queries;
pub mod parser;
pub mod properties;
pub mod selectors;
pub mod str;
pub mod stylesheets;
pub mod stylist;
pub mod used_values;
pub mod values;

pub type LocalName = values::GenericAtomIdent<html5ever::LocalNameStaticSet>;
pub type Namespace = values::GenericAtomIdent<html5ever::NamespaceStaticSet>;
pub type Prefix = values::GenericAtomIdent<html5ever::PrefixStaticSet>;
