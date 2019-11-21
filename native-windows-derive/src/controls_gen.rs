use proc_macro2 as pm2;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;


#[derive(Debug)]
struct ControlParam {
    ident: syn::Ident,
    sep: Token![:],
    e: syn::Expr,
}

impl Parse for ControlParam {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ControlParam {
            ident: input.parse()?,
            sep: input.parse()?,
            e: input.parse()?,
        })
    }
}


#[derive(Debug)]
struct ControlParameters {
    params: Punctuated<ControlParam, Token![,]>
}

impl Parse for ControlParameters {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        Ok(ControlParameters {
            params: content.parse_terminated(ControlParam::parse)?
        })
    }
}


/// Generate the code that inits the control in the `build_ui` function or the `build_partial` function
/// Note that ordering is done in `organize_controls`
pub fn generate_control(field: &syn::Field) -> Option<pm2::TokenStream> {
    let attrs = &field.attrs;
    if attrs.len() == 0 { return None; }

    let member_name = &field.ident.as_ref().expect("Cannot find member name when generating control");

    let attr = match find_control_attr(&attrs) {
        Some(a) => a,
        None => { return None; }
    };

    let control_type = extract_control_type(&field.ty);
    
    let control_tks = match &control_type as &str {
        "Window" => generate_window(&member_name, attr),
        other => panic!("Unkown nwg type #{}. If using user control try `control(ty=Button)`.", other)
    };

    Some(control_tks)
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

fn extract_control_type(ty: &syn::Type) -> String {
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

fn parse_parameters(m: &pm2::Ident, s: &pm2::TokenStream) -> ControlParameters {
    match syn::parse2(s.clone()) {
        Ok(a) => a,
        Err(e) => panic!("Failed to parse field #{}: {}", m, e)
    }
}

fn expand_flags(p: &mut ControlParameters, base: &'static str) {
    let mut flags = p.params.iter_mut().find(|f| &f.ident == "flags");
    if let Some(flags_param) = flags.as_mut() {
        let flags_value = match &flags_param.e {
            syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                syn::Lit::Str(value) => Some(value),
                other => panic!("Compressed flags must str, got {:?}", other)
            },
            _ => None
        };

        if let Some(fv) = flags_value {
            let flags = fv.value();
            let splitted: Vec<&str> = flags.split('|').collect();

            let flags_count = splitted.len() - 1;
            let mut final_flags: String = String::with_capacity(100);
            for (i, value) in splitted.into_iter().enumerate() {
                final_flags.push_str("nwg::");
                final_flags.push_str(base);
                final_flags.push_str("::");
                final_flags.push_str(value);

                if i != flags_count {
                    final_flags.push('|');
                }
            }

            flags_param.e = syn::parse_str(&final_flags).expect("Failed to parse flags");
        }
    }
}

fn generate_window(member: &pm2::Ident, attr: &syn::Attribute) -> pm2::TokenStream {
    let mut control_params = parse_parameters(member, &attr.tokens);
    expand_flags(&mut control_params, "WindowFlags");

    let ids: Vec<&syn::Ident> = control_params.params.iter().map(|p| &p.ident).collect();
    let values: Vec<&syn::Expr> = control_params.params.iter().map(|p| &p.e).collect();

    quote! {
        nwg::Window::builder()
            #(.#ids(#values))*
            .build(&mut data.#member)?;
    }
}
