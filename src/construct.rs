use syn::{
    Result,
    Token,
    Ident,
    punctuated::Punctuated,
    FieldValue,
    parse::{Parse, ParseStream},
};

pub struct Construction {
    struct_name: Ident,
    semicolon: Token![;],
    fields: Punctuated<FieldValue, Token![,]>,
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
