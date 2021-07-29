pub mod animation;
pub mod border;
pub mod color;
pub mod image;
pub mod layout;
pub mod length;
pub mod number;
pub mod percentage;
pub mod position;
pub mod text;
pub mod time;
pub mod url;

/// A CSS float value.
pub type CSSFloat = f32;

#[derive(Clone)]
pub struct CustomIdent(pub String);
