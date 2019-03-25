mod construct;

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, DeriveInput, Ident};
use quote::quote;
use proc_macro_crate::crate_name;

use crate::construct::Construction;

#[proc_macro_derive(VariadicConstructor)]
pub fn variadic_constructor(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let this_crate = crate_name("constructor-macro").unwrap_or("constructor_macro".to_string());
    let this_crate = Ident::new(&this_crate, Span::call_site());

    From::from(quote! {
        #[macro_export]
        macro_rules! #ident {
            ( $( $tokens: tt )* ) => {{
                #this_crate::construct_variadic! {
                    #ident;
                    $($tokens)*
                }
            }};
        }
    })
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
