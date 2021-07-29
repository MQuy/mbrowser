use crate::values::CustomIdent;

#[derive(Clone)]
pub enum KeyframesName {
    None,
    Ident(CustomIdent),
    QuotedString(String),
}

#[derive(Clone)]
pub struct AnimationName {
    names: Vec<KeyframesName>,
}
