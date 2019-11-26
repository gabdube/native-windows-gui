use syn;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use quote::{ToTokens};
use std::collections::HashMap;


#[derive(Debug)]
struct LayoutChildren {

}


#[derive(Debug)]
#[allow(unused)]
struct LayoutAttribute {
    attr_id: syn::Ident,
    sep: Token![:],
    value: syn::Expr,
}

impl Parse for LayoutAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(LayoutAttribute {
            attr_id: input.parse()?,
            sep: input.parse()?,
            value: input.parse()?,
        })
    }
}


#[derive(Debug, Default)]
struct LayoutAttributes {
    params: Punctuated<LayoutAttribute, Token![,]>
}

impl Parse for LayoutAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        Ok(LayoutAttributes {
            params: content.parse_terminated(LayoutAttribute::parse)?
        })
    }
}


#[derive(Debug, Default)]
struct LayoutParams {
    ty: Option<syn::Ident>,
    children: Vec<LayoutChildren>,
    attributes: LayoutAttributes
}


/// Holds layouts data in a UI
pub struct ControlLayouts {
    layouts: HashMap<syn::Ident, LayoutParams>
}

impl ControlLayouts {

    pub fn new() -> ControlLayouts {
        ControlLayouts {
            layouts: HashMap::with_capacity(3)
        }
    }

    pub fn add_layout(&mut self, field: &syn::Field) {
        let attrs = &field.attrs;
        if attrs.len() == 0 { return; }

        let member = field.ident.as_ref().expect("Cannot find member name when generating control");
        let attr = match find_layout_attr(&attrs) {
            Some(a) => a,
            None => { return; }
        };

        let layout = self.layouts
          .entry(member.clone())
          .or_insert(Default::default());

        layout.ty = Some(extract_layout_type(&field.ty));
        parse_layout_params(&member, &attr, &mut layout.attributes);
    }

}


impl ToTokens for ControlLayouts {

    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        let layouts_tk = quote! {};
        layouts_tk.to_tokens(tokens);
    }
    
}


fn find_layout_attr(attrs: &[syn::Attribute]) -> Option<&syn::Attribute> {
    let mut index = None;
    for (i, attr) in attrs.iter().enumerate() {
        if let Some(ident) = attr.path.get_ident() {
            if ident == "nwg_layout" {
                index = Some(i);
                break;
            }
        }
    }

    index.map(|i| &attrs[i])
}

fn parse_layout_params(member: &syn::Ident, layout_attr: &syn::Attribute, attr: &mut LayoutAttributes) {
    let attributes: LayoutAttributes = match syn::parse2(layout_attr.tokens.clone()) {
        Ok(a) => a,
        Err(e) => panic!("Failed to parse layout attributes for #{}: {}", member, e)
    };

    *attr = attributes;
}

fn extract_layout_type(ty: &syn::Type) -> syn::Ident {
    let control_type: String;

    match ty {
        syn::Type::Path(p) => {
            let path_len = p.path.segments.len();
            control_type = p.path.segments[path_len-1].ident.to_string();
        },
        _ => panic!("Ui layout fields must be in a path format `nwg::GridLayout` or simple format `GridLayout`.")
    }

    syn::Ident::new(&control_type, pm2::Span::call_site())
}
