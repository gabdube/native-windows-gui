extern crate proc_macro as pm;
extern crate proc_macro2 as pm2;

#[macro_use]
extern crate syn;
use syn::DeriveInput;
use syn::punctuated::Punctuated;
use syn::token::Comma;

#[macro_use]
extern crate quote;

mod controls;
use controls::ControlGen;

mod events;
use events::ControlEvents;

mod layouts;
use layouts::ControlLayouts;

mod layouts_new;

mod base;
use base::NwgUi;


struct BaseNames {
    n_module: syn::Ident,
    n_partial_module: syn::Ident,
    n_struct: syn::Ident,
    n_struct_ui: syn::Ident,
}

fn to_snake_case(s: &str) -> String {
    let mut snake = String::with_capacity(s.len());

    for (i, c) in s.char_indices() {
        if c.is_ascii_uppercase() {
            if i != 0 {
                snake.push('_');
            }
            snake.push_str(c.to_lowercase().to_string().as_ref());
        } else {
            snake.push(c);
        }
    }

    snake
}

fn parse_base_names(d: &DeriveInput) -> BaseNames {
    let base_name = d.ident.to_string();
    let module_name = format!("{}_ui", to_snake_case(&base_name));
    let partial_module = format!("partial_{}_ui", to_snake_case(&base_name));
    let struct_name = format!("{}Ui", &base_name);

    BaseNames {
        n_module: syn::Ident::new(&module_name, pm2::Span::call_site()),
        n_partial_module: syn::Ident::new(&partial_module, pm2::Span::call_site()),
        n_struct: syn::Ident::new(&base_name, pm2::Span::call_site()),
        n_struct_ui: syn::Ident::new(&struct_name, pm2::Span::call_site()),
    }
}

fn parse_ui_data(d: &DeriveInput) -> Option<&syn::DataStruct> {
    match &d.data {
        syn::Data::Struct(ds) => Some(ds),
        _ => None
    }
} 

fn parse_named_fields(d: &syn::DataStruct) -> Option<&Punctuated<syn::Field, Comma>> {
    match &d.fields {
        syn::Fields::Named(n) => Some(&n.named),
        _ => None
    }
} 

fn generate_build_ui(n: &BaseNames, s: &syn::DataStruct) -> pm2::TokenStream {
    let struct_name = &n.n_struct;
    let ui_struct_name = &n.n_struct_ui;

    let named_fields = parse_named_fields(s).expect("Ui structure must have named fields");

    let mut fields: Vec<ControlGen> = Vec::with_capacity(named_fields.len());
    let mut events = ControlEvents::with_capacity(named_fields.len());
    let mut layouts = ControlLayouts::new();
    for f in named_fields.iter() {
        if let Some(control) = controls::generate_control(f) {
            fields.push(control);
            events.parse(f);
            layouts.add_item(f);
        }

        layouts.add_layout(f);
    }

    controls::organize_controls(&mut fields);
    layouts.organize_layouts();

    quote! {
        fn build_ui(mut data: #struct_name) -> Result<Rc<#ui_struct_name>, nwg::NwgError> {

            #(#fields)*

            let ui = Rc::new(#ui_struct_name { inner: data });

            #events

            #layouts
            
            Ok(ui)
        }
    }
}

#[proc_macro_derive(NwgUi, attributes(nwg_control, nwg_events, nwg_layout, nwg_layout_item, nwg_partial))]
pub fn derive_ui(input: pm::TokenStream) -> pm::TokenStream {
    let base = parse_macro_input!(input as DeriveInput);

    let names = parse_base_names(&base);
    let ui_data = parse_ui_data(&base).expect("NWG derive can only be implemented on structs");

    let build_ui_function = generate_build_ui(&names, &ui_data);
    let module_name = &names.n_module;
    let struct_name = &names.n_struct;
    let ui_struct_name = &names.n_struct_ui;

    let derive_ui = quote! {
        mod #module_name {
            use native_windows_gui as nwg;
            use super::*;
            use std::ops::Deref;
            use std::rc::Rc;
            use std::fmt;

            pub struct #ui_struct_name {
                inner: #struct_name
            }

            impl nwg::NativeUi<#struct_name, #ui_struct_name> for #struct_name {
                #build_ui_function
            }

            impl Deref for #ui_struct_name {
                type Target = #struct_name;
        
                fn deref(&self) -> &#struct_name {
                    &self.inner
                }
            }

            impl fmt::Debug for #ui_struct_name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "[#ui_struct_name Ui]")
                }
            }
        }
    };

    pm::TokenStream::from(derive_ui)
}

#[proc_macro_derive(NwgUi2, attributes(nwg_control, nwg_events, nwg_layout, nwg_layout_item, nwg_partial))]
pub fn derive_ui2(input: pm::TokenStream) -> pm::TokenStream {
    let base = parse_macro_input!(input as DeriveInput);
    let names = parse_base_names(&base);
    let ui_data = parse_ui_data(&base).expect("NWG derive can only be implemented on structs");

    let module_name = &names.n_module;
    let struct_name = &names.n_struct;
    let ui_struct_name = &names.n_struct_ui;

    let ui = NwgUi::build(&ui_data);
    let controls = ui.controls();
    let events = ui.events();
    let layouts = ui.layouts();

    let derive_ui = quote! {
        mod #module_name {
            use super::*;
            use native_windows_gui as nwg;
            use std::ops::Deref;
            use std::rc::Rc;
            use std::fmt;

            pub struct #ui_struct_name {
                inner: #struct_name
            }

            impl nwg::NativeUi<#struct_name, Rc<#ui_struct_name>> for #struct_name {
                fn build_ui(mut data: #struct_name) -> Result<Rc<#ui_struct_name>, nwg::NwgError> {
                    #controls

                    let ui = Rc::new(#ui_struct_name { inner: data });

                    #events
                    #layouts
                    
                    Ok(ui)
                }
            }

            impl Deref for #ui_struct_name {
                type Target = #struct_name;
        
                fn deref(&self) -> &#struct_name {
                    &self.inner
                }
            }

            impl fmt::Debug for #ui_struct_name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "[#ui_struct_name Ui]")
                }
            }
        }
    };

    pm::TokenStream::from(derive_ui)
}

#[proc_macro_derive(NwgPartial, attributes(nwg_control, nwg_events, nwg_layout, nwg_layout_item, nwg_partial))]
pub fn derive_partial(input: pm::TokenStream) -> pm::TokenStream {
    let base = parse_macro_input!(input as DeriveInput);

    let names = parse_base_names(&base);

    let partial_name = &names.n_partial_module;
    let struct_name = &names.n_struct;

    let partial_ui = quote! {
        mod #partial_name {
            use native_windows_gui as nwg;
            use super::*;
        
            impl nwg::PartialUi<#struct_name> for #struct_name {
                fn build_partial<W: Into<nwg::ControlHandle>>(_data: &mut #struct_name, _parent: Option<W>) -> Result<(), nwg::NwgError> {
                    Ok(())
                }

                fn process_event<'a>(&self, _evt: nwg::Event, _evt_data: &nwg::EventData, _handle: nwg::ControlHandle) {
                }

                fn handles(&self) -> Vec<&nwg::ControlHandle> {
                    Vec::new()
                }
            }
        }
    };

    pm::TokenStream::from(partial_ui)
}
