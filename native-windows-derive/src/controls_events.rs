use syn;
use syn::punctuated::Punctuated;
use quote::{ToTokens};
use std::collections::HashMap;


/// A callbacks for a event type
#[derive(Default)]
pub struct EventCallback {

}

/// Wrapper over a basic event dispatcher
pub struct ControlEvents {
    handles: Punctuated<syn::Path, Token![,]>,
    callbacks: HashMap<syn::Ident, EventCallback>
}

impl ControlEvents {

    pub fn with_capacity(cap: usize) -> ControlEvents {
        ControlEvents {
            handles: Punctuated::new(),
            callbacks: HashMap::with_capacity(cap)
        }
    }

}

impl ToTokens for ControlEvents {

    fn to_tokens(&self, tokens: &mut pm2::TokenStream) {
        let handles = &self.handles;

        let mut events = Vec::with_capacity(self.callbacks.len());
        let mut callbacks = Vec::with_capacity(self.callbacks.len());
        for (evt, cb) in self.callbacks.iter() {
            events.push(evt);
            callbacks.push(cb);
        }

        let events_tk = quote! {
            let window_handles: &[&nwg::ControlHandle] = &[#handles];
            for handle in window_handles.iter() {
                let evt_ui = ui.clone();
                let handle_events = move |_evt, _evt_data, _handle| {
                    match _evt { #(
                        Event::#events => {
                        }),*
                        _ => {}
                    }
                };
                nwg::bind_event_handler(handle, handle_events);
            }
        };

        events_tk.to_tokens(tokens);
    }

}

