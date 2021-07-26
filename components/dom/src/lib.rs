pub mod annotation;
pub mod attr;
pub mod cdatasection;
pub mod characterdata;
pub mod comment;
pub mod customelementregistry;
pub mod document;
pub mod documentfragment;
pub mod documenttype;
pub mod element;
pub mod error;
pub mod htmlbaseelement;
pub mod htmlbodyelement;
pub mod htmldivelement;
pub mod htmlelement;
pub mod htmlhtmlelement;
pub mod htmlunknownelement;
pub mod inheritance;
pub mod node;
pub mod nodetype;
pub mod parser;
pub mod str;
pub mod svgelement;
pub mod svggraphicselement;
pub mod svgsvgelement;
pub mod text;
pub mod url;
pub mod virtualmethods;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
