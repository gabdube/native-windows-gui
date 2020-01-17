use quote::{ToTokens};
use crate::layouts::{LayoutChild, BoxLayoutChild, GridLayoutChild, layout_parameters};
use crate::events::ControlEvents;

const TOP_LEVEL: &'static [&'static str] = &[
    "Window", "CanvasWindow", "TabsContainer", "Tab", "MessageWindow"
];


struct NwgControl<'a> {
    id: &'a syn::Ident,
    parent_id: Option<String>,

    ty: &'a syn::Ident,

    layout: Option<LayoutChild>,
    layout_index: usize,

    names: Vec<syn::Ident>,
    values: Vec<syn::Expr>,

    weight: u32,
}

impl<'a> NwgControl<'a> {

    fn valid(field: &syn::Field) -> bool {
        field.attrs.iter().any(|attr| 
            attr.path.get_ident()
                .map(|ident| ident == "nwg_control" )
                .unwrap_or(false)
        )
    }

    fn parse_type(field: &syn::Field) -> &syn::Ident {
        // TODO: extract type from nwg_control first
        
        match &field.ty {
            syn::Type::Path(p) => match p.path.segments.last() {
                Some(seg) => &seg.ident,
                None => panic!("Impossible to parse type for field {:?}. Try specifying it in the nwg_control attribute.", field.ident)
            },
            _ => panic!("Impossible to parse type for field {:?}. Try specifying it in the nwg_control attribute.", field.ident)
        }
    }

    fn expand_flags(&mut self) {
        let flags_index = self.names.iter().position(|n| n == "flags");
        if let Some(i) = flags_index {
            let old_flags = self.values[i].clone();
            self.values[i] = crate::controls::expand_flags(&self.id, &self.ty, old_flags);
        }
    }

    fn expand_parent(&mut self) {
        let parent_index = self.names.iter().position(|n| n == "parent");
        if parent_index.is_none() {
            return;
        }

        let i = parent_index.unwrap();
        let parent_expr: syn::Expr = match &self.values[i] {
            syn::Expr::Path(p) => {
                let id = &p.path.segments.last().unwrap().ident;
                self.parent_id = Some(id.to_string());
                syn::parse_str(&format!("&data.{}", id)).unwrap()
            },
            _ => { panic!("Bad expression type for parent of field {}", self.id); }
        };
        
        self.values[i] = parent_expr;
    }

    fn expand_types(&mut self) {
        let expand_types = ["h_align", "check_state"];
        for t in expand_types.iter() {
            if let Some(i) = self.names.iter().position(|n| n == t) {
                match &mut self.values[i] {
                    syn::Expr::Path(p) =>  {
                        let seg = syn::PathSegment{ 
                            ident: syn::Ident::new("nwg", pm2::Span::call_site()),
                            arguments: syn::PathArguments::None
                        };
                        p.path.segments.insert(0, seg)
                    },
                    _ => {}
                } 
            }
        }
    }

}


struct NwgLayout<'a> {
    id: &'a syn::Ident,
    ty: &'a syn::Ident,
    names: Vec<syn::Ident>,
    values: Vec<syn::Expr>,
}

impl<'a> NwgLayout<'a> {

    fn valid(field: &syn::Field) -> bool {
        field.attrs.iter().any(|attr| 
            attr.path.get_ident()
                .map(|ident| ident == "nwg_layout" )
                .unwrap_or(false)
        )
    }

    fn parse_type(field: &syn::Field) -> &syn::Ident {
        // TODO: extract type from nwg_layout first
        
        match &field.ty {
            syn::Type::Path(p) => match p.path.segments.last() {
                Some(seg) => &seg.ident,
                None => panic!("Impossible to parse type for field {:?}. Try specifying it in the nwg_control attribute.", field.ident)
            },
            _ => panic!("Impossible to parse type for field {:?}. Try specifying it in the nwg_control attribute.", field.ident)
        }
    }

    fn expand_parent(&mut self) {
        let parent_index = self.names.iter().position(|n| n == "parent");
        if parent_index.is_none() {
            return;
        }

        let i = parent_index.unwrap();
        let parent_expr: syn::Expr = match &self.values[i] {
            syn::Expr::Path(p) => {
                let id = &p.path.segments.last().unwrap().ident;
                syn::parse_str(&format!("&ui.{}", id)).unwrap()
            },
            _ => { panic!("Bad expression type for parent of field {}", self.id); }
        };
        
        self.values[i] = parent_expr;
    }

    fn expand_types(&mut self) {
        let type_index = self.names.iter().position(|n| n == "layout_type");
        if let Some(i) = type_index {
            match &mut self.values[i] {
                syn::Expr::Path(p) =>  {
                    let seg = syn::PathSegment{ 
                        ident: syn::Ident::new("nwg", pm2::Span::call_site()),
                        arguments: syn::PathArguments::None
                    };
                    p.path.segments.insert(0, seg)
                },
                _ => {}
            }
        }
    }

}


pub struct NwgUiControls<'a>(&'a NwgUi<'a>);

impl<'a> ToTokens for NwgUiControls<'a> {

    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {

        struct ControlGen<'b> {
            item: &'b NwgControl<'b>
        }

        impl<'b> ToTokens for ControlGen<'b> {
            fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
                let item = &self.item;
                let ty = item.ty;
                let member = item.id;
                let names = &item.names;
                let values = &item.values;
                let control_tk = quote! {
                    nwg::#ty::builder()
                        #(.#names(#values))*
                        .build(&mut data.#member)?;
                };

                control_tk.to_tokens(tokens);
            }
        }

        let ui = &self.0;
        let controls: Vec<ControlGen> = ui.controls.iter()
            .map(|item| ControlGen { item })
            .collect();

        let controls_tk = quote! {
            #(#controls)*
        };

        controls_tk.to_tokens(tokens);
    }

}


pub struct NwgUiEvents<'a>(&'a NwgUi<'a>);

impl<'a> ToTokens for NwgUiEvents<'a> {

    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        self.0.events.to_tokens(tokens);
    }

}


pub struct NwgUiLayouts<'a>(&'a NwgUi<'a>);

impl<'a> ToTokens for NwgUiLayouts<'a> {

    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {

        struct ControlLayout<'b>(&'b NwgControl<'b>);

        impl<'b> ToTokens for ControlLayout<'b> {
            fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
                let c = &self.0;
                let id = &c.id;

                let item_tk = match c.layout {
                    Some(LayoutChild::Grid( GridLayoutChild {col, row, col_span, row_span} )) => 
                        quote! { 
                            child_item(nwg::GridLayoutItem::new(&ui.#id, #col, #row, #col_span, #row_span))
                        },
                    Some(LayoutChild::Box( BoxLayoutChild {cell, cell_span} )) => 
                        quote! { 
                            child_item(nwg::BoxLayoutItem::new(&ui.#id, #cell, #cell_span))
                        },
                    Some(_) => panic!("Unprocessed layout item"),
                    None => panic!("Unfiltered layout item")
                };

                item_tk.to_tokens(tokens);
            }
        }

        struct LayoutGen<'b> {
            layout: &'b NwgLayout<'b>,
            children: Vec<ControlLayout<'b>>
        }

        impl<'b> ToTokens for LayoutGen<'b> {
            fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
                let ty = &self.layout.ty;
                let id = &self.layout.id;
                let names = &self.layout.names;
                let values = &self.layout.values;
                let children = &self.children;

                let layout_tk = quote! {
                    nwg::#ty::builder()
                        #(.#names(#values))*
                        #(.#children)*
                        .build(&ui.#id);
                };
                layout_tk.to_tokens(tokens);
            }
        }

        let ui = &self.0;
        let layouts: Vec<LayoutGen> = ui.layouts.iter().enumerate()
            .map(|(i, layout)| LayoutGen {
                layout,
                children: ui.controls.iter()
                  .filter(|c| c.layout.is_some() && c.layout_index == i)
                  .map(|c| ControlLayout(c) )
                  .collect(),
            })
            .collect();

        let layouts_tk = quote! {
            #(#layouts)*
        };

        layouts_tk.to_tokens(tokens);
    }

}



pub struct NwgUi<'a> {
    controls: Vec<NwgControl<'a>>,
    layouts: Vec<NwgLayout<'a>>,
    events: ControlEvents,
}

impl<'a> NwgUi<'a> {

    pub fn build(data: &'a syn::DataStruct, partial: bool) -> NwgUi<'a> {
        let named_fields = match &data.fields {
            syn::Fields::Named(n) => &n.named,
            _ => panic!("Ui structure must have named fields")
        };
        
        let mut controls = Vec::with_capacity(named_fields.len());
        let mut layouts = Vec::with_capacity(named_fields.len());
        let mut events = ControlEvents::with_capacity(named_fields.len());

        let partial_parent_expr: syn::Expr = syn::parse_str("parent_ref.unwrap()").unwrap();
        let parent_ident = syn::Ident::new("parent", pm2::Span::call_site());

        // First pass: parse controls, layouts, and events
        for field in named_fields {
            if NwgControl::valid(field) {
                let id = field.ident.as_ref().unwrap();
                let ty = NwgControl::parse_type(field);
                let (names, values) = crate::controls::control_parameters(field);

                let f = NwgControl {
                    id,
                    parent_id: None,
                    ty,
                    layout: LayoutChild::prepare(field),
                    layout_index: 0,
                    names,
                    values,
                    weight: 0,
                };

                events.parse(field);

                controls.push(f);
            }

            if NwgLayout::valid(field) {

                let id = field.ident.as_ref().unwrap();
                let ty = NwgLayout::parse_type(field);
                let (names, values) = layout_parameters(field);

                let layout = NwgLayout {
                    id, ty, names, values,
                };

                layouts.push(layout);
            }
        }

        // Parent stuff
        for i in 0..(layouts.len()) {
            let has_attr_parent = layouts[i].names.iter().any(|n| n == "parent");
            if has_attr_parent {
                layouts[i].expand_parent();
            } else {
                if partial {
                    layouts[i].names.push(parent_ident.clone());
                    layouts[i].values.push(partial_parent_expr.clone());
                } else {
                    panic!("Auto detection of layout parent outside of partial is not yet implemented!");
                }  
            }

            // Match the layout item to the layout object
            for control in controls.iter_mut() {
                if let Some(child_layout) = control.layout.as_mut() {
                    let layout = &layouts[i];
                    if child_layout.parent_matches(&layout.id) {
                        child_layout.parse(&layout.ty);
                        control.layout_index = i;
                    }
                }
            }
        }
        
        for i in 0..(controls.len()) {
            let has_attr_parent = controls[i].names.iter().any(|n| n == "parent");
            if has_attr_parent {
                controls[i].expand_parent();
            } else {
                let parent = controls[0..i]
                    .iter().rev()
                    .find(|i| TOP_LEVEL.iter().any(|top| i.ty == top) );
                
                if let Some(parent) = parent {
                    let parent_id = Some(parent.id.to_string());
                    let parent_expr: syn::Expr = syn::parse_str(&format!("&data.{}", parent.id)).unwrap();
                    controls[i].names.push(parent_ident.clone());
                    controls[i].values.push(parent_expr);
                    controls[i].parent_id = parent_id;
                }
            }
        }

        // Parent Weight
        fn compute_weight(controls: &[NwgControl], index: usize, weight: &mut u32) {
            match &controls[index].parent_id {
                Some(p) => 
                    if let Some(parent_index) = controls.iter().position(|c| &c.id == &p) {
                        compute_weight(controls, parent_index, weight);
                        *weight += 1;
                    },
                None => {}
            }
        };

        for i in 0..(controls.len()) {
            let mut weight = 0;
            compute_weight(&controls, i, &mut weight);
            controls[i].weight = weight;
        }

        // Helpers
        for control in controls.iter_mut() {
            control.expand_flags();
            control.expand_types();
        }

        for layout in layouts.iter_mut() {
            layout.expand_types();
        }

        // Sort by weight
        controls.sort_unstable_by(|a, b| a.weight.cmp(&b.weight));

        NwgUi { controls, layouts, events }
    }

    pub fn controls(&self) -> NwgUiControls {
        NwgUiControls(self)
    }

    pub fn events(&self) -> NwgUiEvents {
        NwgUiEvents(self)
    }

    pub fn layouts(&self) -> NwgUiLayouts {
        NwgUiLayouts(self)
    }

}
