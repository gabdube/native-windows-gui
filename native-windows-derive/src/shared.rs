use syn::parse::{Parse, ParseStream, ParseBuffer};
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
        fn maybe_parse_parens<'a>(input: &ParseBuffer<'a>) -> Result<ParseBuffer<'a>, syn::Error> {
            let content;
            parenthesized!(content in input);
            Ok(content)
        }

        let parameters = match maybe_parse_parens(input) {
            Ok(parse_buffer) => {
                Parameters { 
                    params: parse_buffer.parse_terminated(Param::parse)?
                }
            },
            Err(_) => Parameters {
                params: Punctuated::new()
            },
        };

        Ok(parameters)
    }
}
