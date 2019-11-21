extern crate proc_macro as pm;
extern crate proc_macro2 as pm2;

#[macro_use]
extern crate syn;
use syn::DeriveInput;
use syn::punctuated::Punctuated;
use syn::token::Comma;

#[macro_use]
extern crate quote;

mod controls_gen;
use controls_gen::ControlGen;


struct BaseNames {
    n_module: syn::Ident,
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
    let struct_name = format!("{}Ui", &base_name);
    BaseNames {
        n_module: syn::Ident::new(&module_name, pm2::Span::call_site()),
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
    for f in named_fields.iter() {
        if let Some(control) = controls_gen::generate_control(f) {
            fields.push(control);
        }
    }

    controls_gen::organize_controls(&mut fields);

    quote! {
        fn build_ui(mut data: #struct_name) -> Result<Rc<#ui_struct_name>, nwg::SystemError> {

            #(#fields)*

            let ui = Rc::new(#ui_struct_name { inner: data });

            Ok(ui)
        }
    }
}

#[proc_macro_derive(NwgUi, attributes(nwg_control))]
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
