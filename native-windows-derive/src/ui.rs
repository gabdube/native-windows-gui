use quote::{ToTokens};
use crate::layouts::{LayoutChild, FlexboxLayoutChild, GridLayoutChild, layout_parameters};
use crate::events::ControlEvents;
use crate::shared::Parameters;

const TOP_LEVEL: &'static [&'static str] = &[
    "Window", "MessageWindow", "ExternCanvas"
];

const AUTO_PARENT: &'static [&'static str] = &[
    "Window", "TabsContainer", "Tab", "MessageWindow", "ExternCanvas"
];


struct NwgControl<'a> {
    id: &'a syn::Ident,
    parent_id: Option<String>,

    ty: syn::Ident,

    layout: Option<LayoutChild>,
    layout_index: usize,

    names: Vec<syn::Ident>,
    values: Vec<syn::Expr>,

    // First value if the parent order, second value is the insert order
    weight: [u16; 2],
}

impl<'a> NwgControl<'a> {

    fn valid(field: &syn::Field) -> bool {
        field.attrs.iter().any(|attr| 
            attr.path.get_ident()
                .map(|ident| ident == "nwg_control" )
                .unwrap_or(false)
        )
    }

    fn parse_type(field: &syn::Field) -> syn::Ident {
        // Check for `ty` in nwg_control
        let nwg_control = |attr: &&syn::Attribute| {
            attr.path.get_ident()
              .map(|id| id == "nwg_control" )
              .unwrap_or(false)
        };

        let attr = match field.attrs.iter().find(nwg_control) {
            Some(attr) => attr,
            None => unreachable!()
        };

        let params: Parameters = match syn::parse2(attr.tokens.clone()) {
            Ok(p) => p,
            Err(e) => panic!("Failed to parse field #{}: {}", field.ident.as_ref().unwrap(), e)
        };

        match params.params.iter().find(|p| p.ident == "ty").map(|p| &p.e) {
            Some(syn::Expr::Path(p)) => match p.path.segments.last().map(|seg| seg.ident.clone()) {
                Some(ty) => { return ty; }
                None => {}
            },
            _ => {}
        }
        
        // Use field type
        match &field.ty {
            syn::Type::Path(p) => match p.path.segments.last() {
                Some(seg) => seg.ident.clone(),
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

}


struct NwgResource<'a> {
    id: &'a syn::Ident,
    ty: syn::Ident,
    names: Vec<syn::Ident>,
    values: Vec<syn::Expr>,
}

impl<'a> NwgResource<'a> {

    fn valid(field: &syn::Field) -> bool {
        field.attrs.iter().any(|attr| 
            attr.path.get_ident()
                .map(|ident| ident == "nwg_resource" )
                .unwrap_or(false)
        )
    }

    fn parse_type(field: &syn::Field) -> syn::Ident {
        // Check for `ty` in nwg_resource
        let nwg_resource = |attr: &&syn::Attribute| {
            attr.path.get_ident()
              .map(|id| id == "nwg_resource" )
              .unwrap_or(false)
        };

        let attr = match field.attrs.iter().find(nwg_resource) {
            Some(attr) => attr,
            None => unreachable!()
        };

        let params: Parameters = match syn::parse2(attr.tokens.clone()) {
            Ok(p) => p,
            Err(e) => panic!("Failed to parse field #{}: {}", field.ident.as_ref().unwrap(), e)
        };

        match params.params.iter().find(|p| p.ident == "ty").map(|p| &p.e) {
            Some(syn::Expr::Path(p)) => match p.path.segments.last().map(|seg| seg.ident.clone()) {
                Some(ty) => { return ty; }
                None => {}
            },
            _ => {}
        }
        
        // Use field type
        match &field.ty {
            syn::Type::Path(p) => match p.path.segments.last() {
                Some(seg) => seg.ident.clone(),
                None => panic!("Impossible to parse type for field {:?}. Try specifying it in the nwg_resource attribute.", field.ident)
            },
            _ => panic!("Impossible to parse type for field {:?}. Try specifying it in the nwg_resource attribute.", field.ident)
        }
    }

}

#[derive(Debug)]
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

}

struct NwgPartial<'a> {
    id: &'a syn::Ident,
    ty: &'a syn::Ident,
    parent: Option<syn::Ident>,
}


impl<'a> NwgPartial<'a> {
    fn valid(field: &syn::Field) -> bool {
        field.attrs.iter().any(|attr| 
            attr.path.get_ident()
                .map(|ident| ident == "nwg_partial" )
                .unwrap_or(false)
        )
    }

    fn parse_type(field: &syn::Field) -> &syn::Ident {
        match &field.ty {
            syn::Type::Path(p) => match p.path.segments.last() {
                Some(seg) => &seg.ident,
                None => panic!("Impossible to parse type for field {:?}. Try specifying it in the nwg_partial attribute.", field.ident)
            },
            _ => panic!("Impossible to parse type for field {:?}. Try specifying it in the nwg_partial attribute.", field.ident)
        }
    }

    fn parse_parent(field: &syn::Field) -> Option<syn::Ident> {
        let nwg_partial = |attr: &&syn::Attribute| {
            attr.path.get_ident()
              .map(|id| id == "nwg_partial" )
              .unwrap_or(false)
        };

        let attr = match field.attrs.iter().find(nwg_partial) {
            Some(attr) => attr,
            None => unreachable!()
        };

        let params: Parameters = match syn::parse2(attr.tokens.clone()) {
            Ok(p) => p,
            Err(e) => panic!("Failed to parse field #{}: {}", field.ident.as_ref().unwrap(), e)
        };

        let parent_value = params.params.iter().find(|p| p.ident == "parent").map(|p| &p.e);
        match parent_value {
            Some(v) => match v {
                syn::Expr::Path(p) => p.path.segments.last().map(|seg| seg.ident.clone()),
                _ => None,
            },
            None => None
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
                let ty = &item.ty;
                let member = item.id;
                let names = &item.names;
                let values = &item.values;
                let control_tk = quote! {
                    #ty::builder()
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

pub struct NwgUiResources<'a>(&'a NwgUi<'a>);

impl<'a> ToTokens for NwgUiResources<'a> {

    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        
        struct ResourceGen<'b> {
            item: &'b NwgResource<'b>
        }

        impl<'b> ToTokens for ResourceGen<'b> {
            fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
                let item = &self.item;
                let ty = &item.ty;
                let member = item.id;
                let names = &item.names;
                let values = &item.values;
                let resource_tk = quote! {
                    #ty::builder()
                        #(.#names(#values))*
                        .build(&mut data.#member)?;
                };

                resource_tk.to_tokens(tokens);
            }
        }

        let ui = &self.0;
        let resources: Vec<ResourceGen> = ui.resources.iter()
            .map(|item| ResourceGen { item })
            .collect();

        let resources_tk = quote! {
            #(#resources)*
        };

        resources_tk.to_tokens(tokens);
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

                let item_tk = match &c.layout {
                    Some(LayoutChild::Grid( GridLayoutChild {col, row, col_span, row_span} )) => 
                        quote! { 
                            child_item(GridLayoutItem::new(&ui.#id, #col, #row, #col_span, #row_span))
                        },
                    Some(LayoutChild::Flexbox( FlexboxLayoutChild { param_names, param_values } )) => 
                        quote! { 
                            child(&ui.#id)
                            #(.#param_names(#param_values))*
                        },
                    Some(LayoutChild::Init{ field_name, .. }) => panic!("Unmatched layout item for field \"{}\", Did you forget the `layout` parameter?", field_name),
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
                    #ty::builder()
                        #(.#names(#values))*
                        #(.#children)*
                        .build(&ui.#id)?;
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


pub struct NwgUiPartials<'a>(&'a NwgUi<'a>);

impl<'a> ToTokens for NwgUiPartials<'a> {

    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {

        struct PartialGen<'b> {
            item: &'b NwgPartial<'b>
        }

        impl<'b> ToTokens for PartialGen<'b> {
            fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
                let i = &self.item;
                let ty = &i.ty;
                let id = &i.id;
                let parent = &i.parent;

                let partial_tk = if parent.is_none() {
                    quote! {
                        #ty::build_partial::<&Window>(&mut data.#id, None)?;
                    }
                } else {
                    quote! {
                        #ty::build_partial(&mut data.#id, Some(&data.#parent))?;
                    }
                };
                
                partial_tk.to_tokens(tokens);
            }
        }

        let ui = &self.0;
        let partials: Vec<PartialGen> = ui.partials.iter()
            .map(|item| PartialGen { item })
            .collect();

        let partials_tk = quote! {
            #(#partials)*
        };
        
        partials_tk.to_tokens(tokens);
    }

}


pub struct NwgUi<'a> {
    controls: Vec<NwgControl<'a>>,
    resources: Vec<NwgResource<'a>>,
    layouts: Vec<NwgLayout<'a>>,
    partials: Vec<NwgPartial<'a>>,
    events: ControlEvents,
}

impl<'a> NwgUi<'a> {

    pub fn build(data: &'a syn::DataStruct, partial: bool) -> NwgUi<'a> {
        let named_fields = match &data.fields {
            syn::Fields::Named(n) => &n.named,
            _ => panic!("Ui structure must have named fields")
        };
        
        let mut controls = Vec::with_capacity(named_fields.len());
        let mut resources = Vec::with_capacity(named_fields.len());
        let mut layouts = Vec::with_capacity(named_fields.len());
        let mut partials = Vec::with_capacity(named_fields.len());
        let mut events = ControlEvents::with_capacity(partial, named_fields.len());

        let partial_parent_expr: syn::Expr = syn::parse_str("parent_ref.unwrap()").unwrap();
        let parent_ident = syn::Ident::new("parent", pm2::Span::call_site());

        // First pass: parse controls, layouts, and events
        for (field_pos, field) in named_fields.iter().enumerate() {
            if NwgControl::valid(field) {
                let id = field.ident.as_ref().unwrap();
                let ty = NwgControl::parse_type(field);
                let (names, values) = crate::controls::parameters(field, "nwg_control");

                let f = NwgControl {
                    id,
                    parent_id: None,
                    ty,
                    layout: LayoutChild::prepare(field),
                    layout_index: 0,
                    names,
                    values,
                    weight: [0, field_pos as u16],
                };

                events.add_top_level_handle(field);
                events.parse(field);

                controls.push(f);
            }

            if NwgResource::valid(field) {
                let id = field.ident.as_ref().unwrap();
                let ty = NwgResource::parse_type(field);
                let (names, values) = crate::controls::parameters(field, "nwg_resource");
                
                let f = NwgResource {
                    id,
                    ty,
                    names,
                    values,
                };

                resources.push(f);
            }

            else if NwgLayout::valid(field) {
                let id = field.ident.as_ref().unwrap();
                let ty = NwgLayout::parse_type(field);
                let (names, values) = layout_parameters(field);

                let layout = NwgLayout {
                    id, ty, names, values,
                };

                layouts.push(layout);
            }

            else if NwgPartial::valid(field) {
                let partial = NwgPartial {
                    id: field.ident.as_ref().unwrap(),
                    ty: NwgPartial::parse_type(field),
                    parent: NwgPartial::parse_parent(field),
                };

                events.add_partial(&partial.id);
                events.parse(field);

                partials.push(partial);
            }
        }

        // Parent stuff
        for i in 0..(layouts.len()) {
            // Add the parent value of the layout object if it was not already defined
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
            let top_level = TOP_LEVEL.iter().any(|top| &controls[i].ty == top );
            if top_level {
                continue;
            }

            let has_attr_parent = controls[i].names.iter().any(|n| n == "parent");
            if has_attr_parent {
                controls[i].expand_parent();
            } else {
                // Rewind the controls set the parent to the nearest control that supports children
                let parent = controls[0..i]
                    .iter().rev()
                    .find(|i| AUTO_PARENT.iter().any(|top| i.ty == top) );
            
                if let Some(parent) = parent {
                    let parent_id = Some(parent.id.to_string());
                    let parent_expr: syn::Expr = syn::parse_str(&format!("&data.{}", parent.id)).unwrap();
                    controls[i].names.push(parent_ident.clone());
                    controls[i].values.push(parent_expr);
                    controls[i].parent_id = parent_id;
                } else if partial {
                    // If no parent is found, but we are in a partial, use the partial parent.
                    controls[i].names.push(parent_ident.clone());
                    controls[i].values.push(partial_parent_expr.clone());
                    controls[i].parent_id = Some(parent_ident.to_string());
                }
            }
        }

        // Parent Weight
        fn compute_weight(controls: &[NwgControl], index: usize, weight: &mut [u16;2]) {
            match &controls[index].parent_id {
                Some(p) => 
                    if let Some(parent_index) = controls.iter().position(|c| &c.id == &p) {
                        compute_weight(controls, parent_index, weight);
                        weight[0] += 1;
                    },
                None => {}
            }
        }

        for i in 0..(controls.len()) {
            let mut weight = controls[i].weight;
            compute_weight(&controls, i, &mut weight);
            controls[i].weight = weight;
        }

        // Helpers
        for control in controls.iter_mut() {
            control.expand_flags();
        }

        // Sort by weight
        controls.sort_unstable_by(|a, b| {
            let a = ((a.weight[0] as u32) << 16) + (a.weight[1] as u32);
            let b = ((b.weight[0] as u32) << 16) + (b.weight[1] as u32);
            a.cmp(&b)
        });

        NwgUi { controls, resources, layouts, partials, events }
    }

    pub fn controls(&self) -> NwgUiControls {
        NwgUiControls(self)
    }

    pub fn resources(&self) -> NwgUiResources {
        NwgUiResources(self)
    }

    pub fn events(&self) -> NwgUiEvents {
        NwgUiEvents(self)
    }

    pub fn layouts(&self) -> NwgUiLayouts {
        NwgUiLayouts(self)
    }

    pub fn partials(&self) -> NwgUiPartials {
        NwgUiPartials(self)
    }

}
