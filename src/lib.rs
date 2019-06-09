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

use crate::construct::DefaultValue;

#[proc_macro_derive(ConstructorMacro, attributes(default))]
pub fn constructor_macro(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let DeriveInput { ref ident, .. } = input;

    match input.data {
        syn::Data::Struct(ref struct_data) => {
            let tokens = match struct_data.fields {
                syn::Fields::Unit => {
                    quote! {
                        impl ::core::default::Default for #ident {
                            fn default() -> Self {
                                #ident
                            }
                        }

                        #[macro_export]
                        macro_rules! #ident {
                            () => {{ #ident }};
                        }
                    }
                },
                syn::Fields::Named(ref fields_named) => {
                    let defaults = fields_named.named.iter().map(|field| {
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
                    });

                    let (fields, values): (Vec<_>, Vec<_>) = defaults.unzip();
                    quote! {
                        impl ::core::default::Default for #ident {
                            fn default() -> Self {
                                #ident {
                                    #(#fields: #values),*
                                }
                            }
                        }

                        #[macro_export]
                        macro_rules! #ident {
                            ( $( $field: ident : $value: expr, )* ) => {{
                                #ident {
                                    $( $field : $value, )*
                                    ..::core::default::Default::default()
                                }
                            }};
                            ( $( $field: ident : $value: expr ),* ) => {{
                                #ident! {
                                    $( $field : $value, )*
                                }
                            }};
                        }
                    }
                },
                _ => panic!("Tuple structs not supported"),
            };

            tokens.into()
        }
        _ => panic!("Must be a struct"),
    }
}
