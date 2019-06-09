use syn::{
    Result,
    Token,
    Expr,
    parse::{Parse, ParseStream},
};

pub struct DefaultValue {
    pub eq: Token![=],
    pub value: Expr,
}
impl Parse for DefaultValue {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            eq: input.parse()?,
            value: input.parse()?,
        })
    }
}
