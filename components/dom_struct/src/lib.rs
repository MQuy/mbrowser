extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::*;

#[proc_macro_attribute]
pub fn dom_struct(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        panic!("#[dom_struct] takes no arguments");
    }

    let attributes = quote! {
        #[repr(C)]
        #[derive(Clone)]
    };
    // Work around https://github.com/rust-lang/rust/issues/46489
    let attributes: TokenStream = attributes.to_string().parse().unwrap();
    let output: TokenStream = attributes.into_iter().chain(input.into_iter()).collect();
    let item: Item = syn::parse(output).unwrap();

    if let Item::Struct(s) = item {
        let name = &s.ident;
        quote! (
            #s

            impl crate::inheritance::Castable for #name {
            }
        )
        .into()
    } else {
        panic!("#[dom_struct] only applies to structs");
    }
}
