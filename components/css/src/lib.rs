#[macro_use]
extern crate bitflags;

pub mod css_writer;
pub mod element_state;
pub mod error_reporting;
pub mod media_queries;
pub mod parser;
pub mod properties;
pub mod selectors;
pub mod str;
pub mod stylesheets;
pub mod values;

pub type LocalName = crate::values::GenericAtomIdent<html5ever::LocalNameStaticSet>;
pub type Namespace = crate::values::GenericAtomIdent<html5ever::NamespaceStaticSet>;
pub type Prefix = crate::values::GenericAtomIdent<html5ever::PrefixStaticSet>;
