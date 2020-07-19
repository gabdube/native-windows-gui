use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;


#[derive(Debug)]
pub struct Param {
    pub ident: syn::Ident,
    pub sep: Token![:],
    pub e: syn::Expr,
}

impl Parse for Param {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Param {
            ident: input.parse()?,
            sep: input.parse()?,
            e: input.parse()?,
        })
    }
}

#[derive(Debug)]
pub struct Parameters {
    pub params: Punctuated<Param, Token![,]>
}

impl Parse for Parameters {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use syn::group::parse_parens;
        let mut params = None;

        if let Ok(parens) = parse_parens(input) {
            params = Some(parens.content.parse_terminated(Param::parse)?);
        }

        Ok(Parameters {
            params: params.unwrap_or(Punctuated::new())
        })
    }
}
