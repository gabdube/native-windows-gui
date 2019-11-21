use proc_macro2 as pm2;


/// Generate the code that inits the control in the `build_ui` function or the `build_partial` function
/// Note that ordering is done in `organize_controls`
pub fn generate_control(field: &syn::Field) -> Option<pm2::TokenStream> {
    let attrs = &field.attrs;
    if attrs.len() == 0 { return None; }

    let attr = match find_control_attr(&attrs) {
        Some(a) => a,
        None => { return None; }
    };

    let control_type = extract_control_type(&field.ty, attr);

    let c = quote! {

    };

    Some(c)
}

fn find_control_attr(attrs: &[syn::Attribute]) -> Option<&syn::Attribute> {
    let mut index = None;
    for (i, attr) in attrs.iter().enumerate() {
        if let Some(ident) = attr.path.get_ident() {
            if ident == "nwg_control" {
                index = Some(i);
                break;
            }
        }
    }

    index.map(|i| &attrs[i])
}

fn extract_control_type(ty: &syn::Type, attr: &syn::Attribute) -> String {
    let control_type: String;

    match ty {
        syn::Type::Path(p) => {
            let path_len = p.path.segments.len();
            control_type = p.path.segments[path_len-1].ident.to_string();
        },
        _ => panic!("Ui control fields must be in a path format `nwg::Button` or simple format `Button`.")
    }

    control_type
}
