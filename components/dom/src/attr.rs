use std::{cell::RefCell, sync::Arc};

use css::values::length::Length;
use cssparser::RGBA;
use url::Url;

// https://dom.spec.whatwg.org/#interface-att
pub struct Attr {
    name: String,
    value: RefCell<AttrValue>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LengthOrPercentageOrAuto {
    Auto,
    Percentage(f32),
    Length(f32),
}

pub enum AttrValue {
    String(String),
    UInt(String, u32),
    Int(String, i32),
    Double(String, f64),
    Length(String, Option<Length>),
    Color(String, Option<RGBA>),
    Dimension(String, LengthOrPercentageOrAuto),

    /// Stores a URL, computed from the input string and a document's base URL.
    ///
    /// The URL is resolved at setting-time, so this kind of attribute value is
    /// not actually suitable for most URL-reflecting IDL attributes.
    ResolvedUrl(String, Option<Arc<Url>>),
}
