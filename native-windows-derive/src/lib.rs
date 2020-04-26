extern crate proc_macro as pm;
extern crate proc_macro2 as pm2;

#[macro_use]
extern crate syn;
use syn::DeriveInput;

#[macro_use]
extern crate quote;

mod controls;
mod events;
mod layouts;
mod shared;

mod ui;
use ui::NwgUi;


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

#[proc_macro_derive(NwgUi, attributes(nwg_control, nwg_resource, nwg_events, nwg_layout, nwg_layout_item, nwg_partial))]
pub fn derive_ui(input: pm::TokenStream) -> pm::TokenStream {
    let base = parse_macro_input!(input as DeriveInput);
    let names = parse_base_names(&base);
    let ui_data = parse_ui_data(&base).expect("NWG derive can only be implemented on structs");

    let module_name = &names.n_module;
    let struct_name = &names.n_struct;
    let ui_struct_name = &names.n_struct_ui;

    let ui = NwgUi::build(&ui_data, false);
    let controls = ui.controls();
    let resources = ui.resources();
    let partials = ui.partials();
    let layouts = ui.layouts();
    let events = ui.events();

    let derive_ui = quote! {
        mod #module_name {
            use native_windows_gui::*;
            use super::*;
            use std::ops::Deref;
            use std::rc::Rc;
            use std::fmt;

            pub struct #ui_struct_name {
                inner: #struct_name
            }

            impl NativeUi<#struct_name, Rc<#ui_struct_name>> for #struct_name {
                fn build_ui(mut data: #struct_name) -> Result<Rc<#ui_struct_name>, NwgError> {
                    #resources
                    #controls
                    #partials

                    let ui = Rc::new(#ui_struct_name { inner: data });

                    #events
                    #layouts
                    
                    Ok(ui)
                }
            }

            impl #ui_struct_name {
                pub fn destroy(&self) {
                    
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

#[proc_macro_derive(NwgPartial, attributes(nwg_control, nwg_resource, nwg_events, nwg_layout, nwg_layout_item, nwg_partial))]
pub fn derive_partial(input: pm::TokenStream) -> pm::TokenStream {
    let base = parse_macro_input!(input as DeriveInput);

    let names = parse_base_names(&base);

    let partial_name = &names.n_partial_module;
    let struct_name = &names.n_struct;

    let ui_data = parse_ui_data(&base).expect("NWG derive can only be implemented on structs");
    let ui = NwgUi::build(&ui_data, true);
    let controls = ui.controls();
    let resources = ui.resources();
    let layouts = ui.layouts();
    //let events = ui.events();

    let partial_ui = quote! {
        mod #partial_name {
            use native_windows_gui::*;
            use super::*;
        
            impl PartialUi<#struct_name> for #struct_name {

                #[allow(unused)]
                fn build_partial<W: Into<ControlHandle>>(data: &mut #struct_name, _parent: Option<W>) -> Result<(), NwgError> {
                    let parent = _parent.map(|p| p.into());
                    let parent_ref = parent.as_ref();
                    
                    #resources
                    #controls

                    let ui = data;
                    #layouts
                    Ok(())
                }

                fn process_event<'a>(&self, _evt: Event, _evt_data: &EventData, _handle: ControlHandle) {
                }

                fn handles(&self) -> Vec<&ControlHandle> {
                    Vec::new()
                }
            }
        }
    };

    pm::TokenStream::from(partial_ui)
}
