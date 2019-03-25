mod construct;

extern crate proc_macro;
use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;

use crate::construct::Construction;

#[proc_macro_attribute]
pub fn variadic_constructor(attr: TokenStream, item: TokenStream) -> TokenStream {
    unimplemented!()
}

#[proc_macro]
pub fn construct_variadic(tokens: TokenStream) -> TokenStream {
    let Construction {
        struct_name,
        fields,
        ..
    } = parse_macro_input!(tokens as Construction);
    From::from(quote! {
        #struct_name {
            #(#fields,)*
            ..Default::default()
        }
    })
}
