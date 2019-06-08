mod construct;

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{
    parse2,
    parse_quote,
    parse_macro_input,
    DeriveInput,
    Ident,
    Expr,
};
use quote::quote;
use proc_macro_crate::crate_name;

use crate::construct::{Construction, DefaultValue};

#[proc_macro_derive(ConstructorMacro, attributes(default))]
pub fn constructor_macro(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let DeriveInput { ref ident, .. } = input;

    let this_crate = crate_name("constructor-macro").unwrap_or("constructor_macro".to_string());
    let this_crate = Ident::new(&this_crate, Span::call_site());

    match input.data {
        syn::Data::Struct(ref struct_data) => {
            let defaults = match struct_data.fields {
                syn::Fields::Unit => {
                    unimplemented!("Unit struct")
                },
                syn::Fields::Named(ref fields_named) => {
                    fields_named.named.iter().map(|field| {
                        let ident = field.ident.as_ref().unwrap();
                        let mut default_value: (&Ident, Expr) = (ident, parse_quote! {
                            ::core::default::Default::default()
                        });
                        for attr in field.attrs.iter() {
                            let tts = attr.tts.clone();
                            let attr: syn::Result<DefaultValue> = parse2(tts);

                            if let Ok(attr) = attr {

                                let value = attr.value;
                                default_value = (&ident, value);
                            }
                        }
                        default_value
                    })
                },
                _ => panic!("Tuple structs not supported"),
            };
            let (fields, values): (Vec<_>, Vec<_>) = defaults.unzip();

            From::from(quote! {
                impl ::core::default::Default for #ident {
                    fn default() -> Self {
                        #ident {
                            #(#fields: #values),*
                        }
                    }
                }
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
        _ => panic!("Must be a struct"),
    }
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
