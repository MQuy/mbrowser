mod annotation;
mod attr;
mod cdatasection;
mod characterdata;
mod comment;
mod document;
mod documenttype;
mod element;
mod error;
mod htmlbodyelement;
mod htmldivelement;
mod htmlelement;
mod inheritance;
mod node;
mod nodetype;
mod parser;
mod svgelement;
mod svggraphicselement;
mod svgsvgelement;
mod text;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
