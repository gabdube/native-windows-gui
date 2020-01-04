use syn;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use quote::{ToTokens};


//
// Shared stuff between LayoutChild and LayoutAttribute
//

#[derive(Debug, Copy, Clone)]
pub enum LayoutType {
    Unknown,
    BoxLayout,
    GridLayout,
}

impl Default for LayoutType {
    fn default() -> LayoutType { LayoutType::Unknown }
}


#[derive(Debug)]
#[allow(unused)]
struct Attribute {
    attr_id: syn::Ident,
    sep: Token![:],
    value: syn::Expr,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Attribute {
            attr_id: input.parse()?,
            sep: input.parse()?,
            value: input.parse()?,
        })
    }
}

#[derive(Debug, Default)]
struct AttributeCollection {
    params: Punctuated<Attribute, Token![,]>
}

impl Parse for AttributeCollection {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        Ok(AttributeCollection {
            params: content.parse_terminated(Attribute::parse)?
        })
    }
}


//
// Layout children
//

#[derive(Debug)]
struct LayoutChild {
    layout_type: LayoutType,
    member: syn::Ident,
    col: u32,
    row: u32,
    col_span: u32,
    row_span: u32
}

impl ToTokens for LayoutChild {

    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        let member = &self.member;

        let item_tk = match self.layout_type {
            LayoutType::GridLayout => {
                let [c, r, cs, rs] = [self.col, self.row, self.col_span, self.row_span];
                quote! { 
                    child_item(nwg::GridLayoutItem::new(&ui.#member, #c, #r, #cs, #rs))
                }
            },
            LayoutType::BoxLayout => {
                let [cell, span] = [self.col, self.col_span];
                quote! { 
                    child_item(nwg::BoxLayoutItem::new(&ui.#member, #cell, #span))
                }
            },
            _ => panic!("LayoutType for item {:?} was not set", self)
        };
        
        item_tk.to_tokens(tokens);
    }
    
}


//
// Layout attributes
//

struct LayoutAttribute<'a>(&'a Attribute);

impl<'a> ToTokens for LayoutAttribute<'a> {

    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        let attr = self.0;
        let id = &attr.attr_id;
        let val = &attr.value;
        let layouts_tk = quote! { #id(#val) };
        layouts_tk.to_tokens(tokens);
    }
    
}


//
// Global layout parameters
//

#[derive(Debug, Default)]
struct LayoutParams {
    layout_type: LayoutType,
    member: Option<syn::Ident>,
    ty: Option<syn::Ident>,
    children: Vec<LayoutChild>,
    attributes: AttributeCollection
}

impl ToTokens for LayoutParams {

    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        let ty = &self.ty;
        let member = &self.member;
        let children = &self.children;
        let attr: Vec<LayoutAttribute> = self.attributes.params.iter()
            .map(|attr| LayoutAttribute(attr))
            .collect();

        let layouts_tk = quote! { 
            nwg::#ty::builder()
                #(.#attr)*
                #(.#children)*
                .build(&ui.#member);
        };
        layouts_tk.to_tokens(tokens);
    }
    
}


///
/// Holds parsed layouts definitions
///
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

        let member = field.ident.as_ref().expect("Cannot find member name when generating layout");
        let attr = match find_layout_attr(&attrs, "nwg_layout") {
            Some(a) => a,
            None => { return; }
        };

        let mut attributes = AttributeCollection::default();
        parse_attr_collection(&member, &attr, &mut attributes);

        let layout_type = parse_layout_type(&field.ty);

        self.layouts.push(LayoutParams {
            layout_type,
            member: Some(member.clone()),
            ty: Some(extract_layout_type(&field.ty)),
            children: Vec::with_capacity(5),
            attributes,
        });
    }

    pub fn add_item(&mut self, field: &syn::Field) {
        let attrs = &field.attrs;
        if attrs.len() == 0 { return; }

        let member = field.ident.as_ref().expect("Cannot find member name when generating layout item");
        let attr = match find_layout_attr(&attrs, "nwg_layout_item") {
            Some(a) => a,
            None => { return; }
        };

        let mut col = AttributeCollection::default();
        parse_attr_collection(&member, &attr, &mut col);

        let parent_name = layout_item_parent(&member, &col, &self.layouts);
        let mut layout_item = parse_layout_item(&member, &col);

        let parent_layout = self.layouts.iter_mut().find(|l| l.member.as_ref() == Some(&parent_name));
        match parent_layout {
            Some(pl) => {
                layout_item.layout_type = pl.layout_type;
                pl.children.push(layout_item);
            },
            None => {
                // The parent layout might not have been defined yet.
                // In such case, create new layout and add the children
                let lp = LayoutParams {
                    layout_type: LayoutType::Unknown,
                    member: Some(parent_name),
                    ty: None,
                    children: vec![layout_item],
                    attributes: Default::default(),
                };

                self.layouts.push(lp)
            }
        }
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


/// Find the layout attr in the field attributes
fn find_layout_attr<'a>(attrs: &'a[syn::Attribute], name: &'static str) -> Option<&'a syn::Attribute> {
    let mut index = None;
    for (i, attr) in attrs.iter().enumerate() {
        if let Some(ident) = attr.path.get_ident() {
            if ident == name {
                index = Some(i);
                break;
            }
        }
    }

    index.map(|i| &attrs[i])
}

/// Parse the layout parameters
fn parse_attr_collection(member: &syn::Ident, layout_attr: &syn::Attribute, attr: &mut AttributeCollection) {
    let mut attributes: AttributeCollection = match syn::parse2(layout_attr.tokens.clone()) {
        Ok(a) => a,
        Err(e) => panic!("Failed to parse layout attributes for #{}: {}", member, e)
    };

    // Appends `nwg` before the layout type attribute value
    if let Some(attr) = attributes.params.iter_mut().find(|attr| &attr.attr_id == "layout_type") {
        match &mut attr.value {
            syn::Expr::Path(p) => {
                let first = p.path.segments.first().map(|seg| &seg.ident);
                if first.is_some() && first.unwrap() != "nwg" {
                    let seg = syn::PathSegment{ 
                        ident: syn::Ident::new("nwg", pm2::Span::call_site()),
                        arguments: syn::PathArguments::None
                    };
                    p.path.segments.insert(0, seg);
                }
            },
            _ => {}
        }
    }

    *attr = attributes;
}

/// Extract the layout type
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

/// Expand user defined parent field. Ex: "parent: window" becomes "parent: &ui.window"
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
        None => panic!("Layout \"{}\" does not have a parent!", member_name)
    }
}

/// Find or guess the layout parent for a layout item
fn layout_item_parent(mem: &syn::Ident, col: &AttributeCollection, layouts: &[LayoutParams]) -> syn::Ident {
    let parent = col.params.iter().find(|attr| attr.attr_id == &"layout");
    match parent {
        Some(p) => match &p.value {
            syn::Expr::Path(p) => {
                let path_len = p.path.segments.len();
                p.path.segments[path_len-1].ident.clone()
            },
            _ => panic!("Wrong parent value for layout item {}. Parent must be defined this way: `layout: my_layout`.", mem)
        },
        None => {
            let parent_layout = layouts.iter()
                .rev()
                .find(|l| l.member.is_some())
                .map(|l| l.member.as_ref().unwrap());

            match parent_layout {
                Some(pl) => pl.clone(),
                None => panic!("No parent layout for layout item {}! Be sure to define `layout: my_layout` in the parameters", mem)
            }
        }
    }
}

/// Parse a layout child from a AttributeCollection
fn parse_layout_item(mem: &syn::Ident, attr_col: &AttributeCollection) -> LayoutChild {
    let [mut col, mut row, mut col_span, mut row_span] = [0, 0, 1, 1];

    let int_value = |expr: &syn::Expr| -> u32 {
        match expr {
            syn::Expr::Lit(lit) => 
                match &lit.lit {
                    syn::Lit::Int(i) => { i.base10_parse().unwrap() },
                    _ => panic!("Layout item member {} must be a int literal.", mem)
                },
            _ => panic!("Layout item member {} must be a int literal.", mem)
        }
    };

    for p in attr_col.params.iter() {
        let attr_name = p.attr_id.to_string();
        match &attr_name as &str {
            // Grid layout
            "col" => { col = int_value(&p.value) },
            "row" => { row = int_value(&p.value) },
            "col_span" => { col_span = int_value(&p.value) },
            "row_span" => { row_span = int_value(&p.value) },

            // Box layout
            "cell" => { col = int_value(&p.value) },
            "cell_span" => { col_span = int_value(&p.value) },
            _ => {}
        }
    }

    LayoutChild {
        layout_type: LayoutType::Unknown,
        member: mem.clone(),
        col,
        row,
        col_span,
        row_span
    }
}

/// Parse the layout type
fn parse_layout_type(ty: &syn::Type) -> LayoutType {
    match ty {
        syn::Type::Path(p) => {
            let path = &p.path;
            let seg = path.segments.last().unwrap();
            let name = &seg.ident;
            if name == "GridLayout" {
                LayoutType::GridLayout
            } else if name == "BoxLayout" {
                LayoutType::BoxLayout
            } else {
                panic!("Unknown layout type: {}", name);
            }
        },
        t => panic!("Expected Path type got: {:?}", t)
    }
}

