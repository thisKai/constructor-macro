use proc_macro2::TokenStream;
use syn::{
    Result,
    Token,
    Ident,
    punctuated::Punctuated,
    FieldValue,
    parse::{Parse, ParseStream},
};
use quote::{quote, ToTokens, TokenStreamExt};

pub struct Construction {
    pub struct_name: Ident,
    pub semicolon: Token![;],
    pub fields: Punctuated<FieldValue, Token![,]>,
}
impl Parse for Construction {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            struct_name: input.parse()?,
            semicolon: input.parse()?,
            fields: input.parse_terminated(FieldValue::parse)?,
        })
    }
}
