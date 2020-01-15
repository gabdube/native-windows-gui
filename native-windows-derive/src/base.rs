use syn::punctuated::Punctuated;
use syn::token::Comma;


#[derive(Debug)]
pub struct NwgItem {
    id: syn::Ident
}

impl NwgItem {

    pub fn valid(field: &syn::Field) -> bool {
        field.attrs.iter().any(|attr| 
            attr.path.get_ident()
                .map(|ident| ident == "nwg_control" || ident == "nwg_partial" )
                .unwrap_or(false)
        )
    }

}

pub struct NwgUi {
    pub controls: Vec<NwgItem>,
}

impl NwgUi {

    pub fn build(data: &syn::DataStruct) -> NwgUi {
        let named_fields = parse_named_fields(data).expect("Ui structure must have named fields");
        let mut controls: Vec<NwgItem> = Vec::with_capacity(named_fields.len());

        for field in named_fields {
            if !NwgItem::valid(field) {
                continue;
            }

            let f = NwgItem {
                id: field.ident.clone().unwrap(),
            };

            controls.push(f);
        }
        
        println!("{:#?}", controls);

        NwgUi { controls }
    }

}

fn parse_named_fields(d: &syn::DataStruct) -> Option<&Punctuated<syn::Field, Comma>> {
    match &d.fields {
        syn::Fields::Named(n) => Some(&n.named),
        _ => None
    }
} 
