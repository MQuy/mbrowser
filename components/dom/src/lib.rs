mod annotation;
mod attr;
mod cdatasection;
mod characterdata;
mod comment;
mod customelementregistry;
mod document;
mod documenttype;
mod element;
mod error;
mod htmlbaseelement;
mod htmlbodyelement;
mod htmldivelement;
mod htmlelement;
mod htmlhtmlelement;
mod htmlunknownelement;
mod inheritance;
mod node;
mod nodetype;
mod parser;
mod str;
mod svgelement;
mod svggraphicselement;
mod svgsvgelement;
mod text;
mod url;
mod virtualmethods;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
