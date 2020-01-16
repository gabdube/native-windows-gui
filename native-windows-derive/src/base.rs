use quote::{ToTokens};


#[derive(Debug)]
pub struct NwgItem<'a> {
    id: &'a syn::Ident,
    ty: &'a syn::Ident,

    names: Vec<syn::Ident>,
    values: Vec<syn::Expr>,

    weight: u32,
}

impl<'a> NwgItem<'a> {

    fn valid(field: &syn::Field) -> bool {
        field.attrs.iter().any(|attr| 
            attr.path.get_ident()
                .map(|ident| ident == "nwg_control" || ident == "nwg_partial" )
                .unwrap_or(false)
        )
    }

    fn extract_type(field: &syn::Field) -> &syn::Ident {
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

}


pub struct NwgUiControls<'a>(&'a NwgUi<'a>);

impl<'a> ToTokens for NwgUiControls<'a> {

    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {

        struct ControlGen<'b> {
            item: &'b NwgItem<'b>
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
        let events_tk = quote! {

        };

        events_tk.to_tokens(tokens);
    }

}


pub struct NwgUiLayouts<'a>(&'a NwgUi<'a>);

impl<'a> ToTokens for NwgUiLayouts<'a> {

    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        let layouts_tk = quote! {

        };

        layouts_tk.to_tokens(tokens);
    }

}



pub struct NwgUi<'a> {
    pub data: &'a syn::DataStruct,
    pub controls: Vec<NwgItem<'a>>,
}

impl<'a> NwgUi<'a> {

    pub fn build(data: &'a syn::DataStruct) -> NwgUi<'a> {
        let named_fields = match &data.fields {
            syn::Fields::Named(n) => &n.named,
            _ => panic!("Ui structure must have named fields")
        };
        
        let mut controls: Vec<NwgItem> = Vec::with_capacity(named_fields.len());

        // First pass: names & default values
        for field in named_fields {
            if !NwgItem::valid(field) { continue; }

            let (names, values) = crate::controls::control_parameters(field);

            let f = NwgItem {
                id: field.ident.as_ref().unwrap(),
                ty: NwgItem::extract_type(field),
                names,
                values,
                weight: 0,
            };

            controls.push(f);
        }

        // Second pass: Helpers
        for i in 0..(controls.len()) {
            controls[i].expand_flags();
        }

        // Third pass: sort by weight
        controls.sort_unstable_by(|a, b| a.weight.cmp(&b.weight));

        //println!("{:#?}", controls);

        NwgUi { data, controls }
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
