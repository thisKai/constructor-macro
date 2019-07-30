use syn::{
    Result,
    Expr,
    parenthesized,
    token::Paren,
    parse::{Parse, ParseStream},
};

pub struct DefaultValue {
    pub paren: Paren,
    pub value: Expr,
}
impl Parse for DefaultValue {
    fn parse(input: ParseStream) -> Result<Self> {
        dbg!(&input);
        let content;
        Ok(Self {
            paren: parenthesized!(content in input),
            value: content.parse()?,
        })
    }
}
