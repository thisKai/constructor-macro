mod construct;

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn variadic_constructor(attr: TokenStream, item: TokenStream) -> TokenStream {
    unimplemented!()
}

#[proc_macro]
pub fn construct_variadic(tokens: TokenStream) -> TokenStream {
    unimplemented!()
}
