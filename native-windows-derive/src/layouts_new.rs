use syn::punctuated::Punctuated;
use syn::parse::{Parse, ParseStream};


pub struct GridLayoutChild {
    col: u32,
    row: u32,
    col_span: u32,
    row_span: u32
}

pub struct BoxLayoutChild {
    cell: u32,
    cell_span: u32,
}

pub enum LayoutChild {
    Grid(GridLayoutChild),
    Box(BoxLayoutChild)
}

impl LayoutChild {

    pub fn parse(field: &syn::Field) -> Option<LayoutChild> {
        None
    }

}

//
// Main layout
//

#[allow(unused)]
struct LayoutParam {
    ident: syn::Ident,
    sep: Token![:],
    e: syn::Expr,
}

impl Parse for LayoutParam {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(LayoutParam {
            ident: input.parse()?,
            sep: input.parse()?,
            e: input.parse()?,
        })
    }
}


struct LayoutParameters {
    params: Punctuated<LayoutParam, Token![,]>
}

impl Parse for LayoutParameters {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use syn::group::parse_parens;
        let mut params = None;

        if let Ok(parens) = parse_parens(input) {
            params = Some(parens.content.parse_terminated(LayoutParam::parse)?);
        }

        Ok(LayoutParameters {
            params: params.unwrap_or(Punctuated::new())
        })
    }
}

pub fn layout_parameters(field: &syn::Field) -> (Vec<syn::Ident>, Vec<syn::Expr>) {
    let member = match field.ident.as_ref() {
        Some(m) => m,
        None => unreachable!()
    };

    let nwg_layout = |attr: &&syn::Attribute| {
        attr.path.get_ident()
          .map(|id| id == "nwg_layout" )
          .unwrap_or(false)
    };

    let attr = match field.attrs.iter().find(nwg_layout) {
        Some(attr) => attr,
        None => unreachable!()
    };

    let layout: LayoutParameters = match syn::parse2(attr.tokens.clone()) {
        Ok(a) => a,
        Err(e) => panic!("Failed to parse field #{}: {}", member, e)
    };

    let params = layout.params;
    let mut names = Vec::with_capacity(params.len());
    let mut exprs = Vec::with_capacity(params.len());

    for p in params {
        names.push(p.ident);
        exprs.push(p.e);
    }

    (names, exprs)
}
