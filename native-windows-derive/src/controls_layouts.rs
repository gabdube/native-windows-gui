use syn;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use quote::{ToTokens};


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

impl ToTokens for LayoutAttribute {

    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        let id = &self.attr_id;
        let val = &self.value;
        let layouts_tk = quote! { #id(#val) };
        layouts_tk.to_tokens(tokens);
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
    member: Option<syn::Ident>,
    ty: Option<syn::Ident>,
    children: Vec<LayoutChildren>,
    attributes: LayoutAttributes
}

impl ToTokens for LayoutParams {

    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        let ty = &self.ty;
        let member = &self.member;
        let attr: Vec<&LayoutAttribute> = self.attributes.params.iter().collect();

        let layouts_tk = quote! { 
            nwg::#ty::builder()
                #(.#attr)*
                .build(&ui.#member);
        };
        layouts_tk.to_tokens(tokens);
    }
    
}


/// Holds layouts data in a UI
pub struct ControlLayouts {
    layouts: Vec<LayoutParams>
}

impl ControlLayouts {

    pub fn new() -> ControlLayouts {
        ControlLayouts {
            layouts: Vec::with_capacity(3)
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

        let mut layout_params = LayoutParams {
            member: Some(member.clone()),
            ty: Some(extract_layout_type(&field.ty)),
            children: Vec::with_capacity(5),
            attributes: Default::default(),
        };
        parse_layout_params(&member, &attr, &mut layout_params.attributes);
        self.layouts.push(layout_params);
        
    }

    pub fn organize_layouts(&mut self) {
        for layout in self.layouts.iter_mut() {
            expand_parent(layout);
        }
    }

}


impl ToTokens for ControlLayouts {

    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        let layouts = &self.layouts;
        let layouts_tk = quote! { #(#layouts);* };
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

fn expand_parent(layout: &mut LayoutParams) {
    let member_name = layout.member.as_ref().unwrap();
    let parent_attr = layout.attributes.params.iter_mut().find(|p| &p.attr_id == "parent");

    match parent_attr {
        Some(parent) => {
            let parent_name = match &parent.value {
                syn::Expr::Path(p) => {
                    let path_len = p.path.segments.len();
                    p.path.segments[path_len-1].ident.to_string()
                },
                _ => panic!("Bad parent value for layout {}", member_name)
            };

            let final_parent = format!("&ui.{}", parent_name);
            parent.value = match syn::parse_str(&final_parent) {
                Ok(e) => e,
                Err(e) => panic!("Failed to parse parent value for layout {}: {}", member_name, e)
            };
        },
        None => panic!("Layout {} does not have a parent!", member_name)
    }
}
