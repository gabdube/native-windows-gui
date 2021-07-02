extern crate proc_macro as pm;
extern crate proc_macro2 as pm2;

#[macro_use]
extern crate syn;
use syn::{DeriveInput, GenericParam, TypeParam, LifetimeDef};
use syn::punctuated::Punctuated;

#[macro_use]
extern crate quote;

use proc_macro_crate::crate_name;

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

/// Extract generic names from definition.
/// It is useful to erase definition and generate `impl<T: Trait1> Struct<T> {...}` tokens.
///
/// For example `<'a: 'b, T: Trait1, const C: usize = 10>` becomes `<'a, T, C>`
fn extract_generic_names(generics: &Punctuated<GenericParam, Token![,]>) -> Punctuated<GenericParam, Token![,]> {
    let mut generic_names: Punctuated<GenericParam, Token![,]> = Punctuated::new();
    for generic_param in generics {
        let ident = match generic_param {
            GenericParam::Type(t) => GenericParam::Type(TypeParam::from(t.ident.clone())),
            GenericParam::Lifetime(l) => GenericParam::Lifetime(LifetimeDef::new(l.lifetime.clone())),
            GenericParam::Const(c) => GenericParam::Type(TypeParam::from(c.ident.clone())), // a little hack
        };
        generic_names.push(ident);
    }
    generic_names
}

/**

The `NwgUi` macro implements the native-windows-gui `NativeUi` trait on the selected struct

For a detailed documentation of this macro see the documentation "native-windows-docs/nwd_basics.html"


# Usage

```rust
use native_windows_gui as nwg;

#[derive(NwgUi, Default)]
pub struct BasicApp {
    #[nwg_control(title: "Window")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()] )]
    window: nwg::Window,

    #[nwg_resource(family: "Arial")]
    font: nwg::Font,

    #[nwg_layout(parent: window)]
    my_layout: nwg::GridLayout,

    #[nwg_control(text: "Button")]
    #[nwg_layout_item(layout: my_layout, col: 0, row: 0)]
    button: nwg::Button,
}

// ...

let my_ui = BasicAppUi::build_ui(Default::default()).unwrap();
```

The macro creates a new struct named `[StructName]Ui` in a submodule named `[struct_name]_ui`.

The trait `NativeUi` is implemented on this struct and the boilerplate code is generated for every field tagged by attributes.
Fields without attributes, even `nwg` types, are left untouched.

Finally, the derive macro also creates a default event handler that will live through the ui struct lifetime. 


# Attributes usage

Actual UI creation works by tagging the struct fields with the some attributes

## Controls

Use the `nwg_control` attribute to instance a control from a struct field:

```
nwg_control(builder_field: builder_value,*)
```

This syntax is basically a compressed version of the nwg control builders. The control attribute
also has built-in helpers: auto parent detection and compressed flags syntax (see the docs for more info on these features).

```
#[nwg_control(text: "Heisenberg", size: (280, 25), position: (10, 10))]
name_edit: nwg::TextInput,

// is the same as 

nwg::TextInput::builder()
    .text("Heisenberg")
    .size((280, 25))
    .position((10, 10))
    .build(&mut data.text_edit);
```

## Resources

Use the `nwg_resource` to generate a resource from a struct field. It works the exact same way as `nwg_controls`. 
Resources are always instanced before the controls.

## Events

Use the `nwg_events` attribute to add events to the default event handler. Events can only be applied to a field that
was tagged with `nwg_control`.

```
nwg_events( EVENT_TYPE: [CALLBACK(ARGS),*] )
```

where:
 - **EVENT_TYPE** is any value of the Event enum.
 - **CALLBACK** is the function that will be called when the event is triggered.
 - **ARGS** specifies the parameters of the callback (optional).

## Events arguments

By default, native windows derive assumes the callback is a method of the Ui structure. So for example, 
`TestApp::callback1` assumes the method has the following signature `callback1(&self)`.

That's very limiting. For example, if the same callback is used by two different controls, there's no way to differenciate them. In order to fix this, NWD lets you define the callbacks parameters using those identifiers:

 - **SELF**: Sends the ui struct `&UiStruct`. If there are no parameters, this is the default.
 - **RC_SELF**: Sends the rc ui struct `&Rc<UiStruct>`. Useful for binding dynamic events
 - **CTRL**: Sends the control that triggered the event. Ex: `&Button`
 - **HANDLE**: Sends the handle of the control. `&ControlHandle`
 - **EVT**: Sends the event that was triggered. `&Event`
 - **EVT_DATA**: Sends the data of the event that was triggered. `&EventData`

It's also possible to not use any parameters, ex: `TestApp::callback1()`. 

Different event types:

```
struct TestApp {
    #[nwg_control]
    #[nwg_events(
        OnButtonClick: [TestApp::callback1, TestApp::callback2],
        OnMouseMove: [TestApp::callback3(SELF, CTRL)],
        OnButtonDoubleClick: [callback, another_callback()]
    )]
    button: nwg::Button
}

fn callback(me: &TestApp) {}
fn another_callback() {}

impl TestApp {
    fn callback1(&self) { }
    fn callback2(&self) { }
    fn callback3(&self, ctrl: &nwg::Button) { }
}
```

## Layouts

Use the `nwg_layout` attribute to instance a layout from a struct field and `nwg_layout_item` to associate a control to a layout.

Under the hood, both these attribute work the same way as `nwg_control`. `nwg_layout` uses the builder attribute for a the layout struct and
`nwg_layout_item` uses the parameters of the item type of the parent (ex: `GridLayoutItem` for `GridLayout`).

NWD cannot guess the parent of layout items.

## Partials

Use the `nwg_partial` attribute to instance a partial from a struct field:

If parts of your UI is another struct that implements the `PartialUi` trait, it can be easily included in your base UI using `nwg_partial`.
The attribute accepts an optional parameter "parent" to pass a parent control to the partial initializer. Unlike the parent in `nwg_controls`,
it must be explicitly defined.

nwg_partial works by calling `PartialUi::build_partial` after initializing the controls of the base UI, calling `PartialUi::process_event` in the default event handler,
and binds the default handler to the handles returned by `PartialUi::handles`

Also see `NwgPartial` for the macro to generate a nwg partial.

```
struct Ui {
    window: nwg::Window,

    #[nwg_partial(parent: window)]
    partial: MyPartial
}
```

*/
#[proc_macro_derive(NwgUi, attributes(nwg_control, nwg_resource, nwg_events, nwg_layout, nwg_layout_item, nwg_partial))]
pub fn derive_ui(input: pm::TokenStream) -> pm::TokenStream {
    let base = parse_macro_input!(input as DeriveInput);
    let names = parse_base_names(&base);
    let ui_data = parse_ui_data(&base).expect("NWG derive can only be implemented on structs");

    let module_name = &names.n_module;
    let struct_name = &names.n_struct;
    let ui_struct_name = &names.n_struct_ui;

    let lt = &base.generics.lt_token;
    let generic_params = &base.generics.params;
    let generic_names = extract_generic_names(generic_params);
    let gt = &base.generics.gt_token;
    let where_clause = &base.generics.where_clause;

    let generics = quote! { #lt #generic_params #gt }; // <'a: 'b, T: Trait1, const C>
    let generic_names = quote! { #lt #generic_names #gt }; // <'a, T, C>

    let ui = NwgUi::build(&ui_data, false);
    let controls = ui.controls();
    let resources = ui.resources();
    let partials = ui.partials();
    let layouts = ui.layouts();
    let events = ui.events();

    let nwg_name = crate_name("native-windows-gui");

    // Returns an error in the examples, so we try a default value
    let nwg = match nwg_name {
        Ok(name) => syn::Ident::new(&name, proc_macro2::Span::call_site()),
        Err(_) => syn::Ident::new("native_windows_gui", proc_macro2::Span::call_site()),   
    };

    let derive_ui = quote! {
        mod #module_name {
            extern crate #nwg as nwg;
            use nwg::*;
            use super::*;
            use std::ops::Deref;
            use std::cell::RefCell;
            use std::rc::Rc;
            use std::fmt;

            pub struct #ui_struct_name #generics #where_clause {
                inner: Rc<#struct_name #generic_names>,
                default_handlers: RefCell<Vec<EventHandler>>
            }

            impl #generics NativeUi<#ui_struct_name #generic_names> for #struct_name #generic_names #where_clause {
                fn build_ui(mut data: Self) -> Result<#ui_struct_name #generic_names, NwgError> {
                    #resources
                    #controls
                    #partials

                    let inner = Rc::new(data);
                    let ui = #ui_struct_name { inner: inner.clone(), default_handlers: Default::default() };

                    #events
                    #layouts
                    
                    Ok(ui)
                }
            }

            impl #generics Drop for #ui_struct_name #generic_names #where_clause {
                /// To make sure that everything is freed without issues, the default handler must be unbound.
                fn drop(&mut self) {
                    let mut handlers = self.default_handlers.borrow_mut();
                    for handler in handlers.drain(0..) {
                        nwg::unbind_event_handler(&handler);
                    }
                }
            }

            impl #generics Deref for #ui_struct_name #generic_names #where_clause {
                type Target = #struct_name #generic_names;

                fn deref(&self) -> &Self::Target {
                    &self.inner
                }
            }

            impl #generics fmt::Debug for #ui_struct_name #generic_names #where_clause {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "[#ui_struct_name Ui]")
                }
            }
        }
    };

    pm::TokenStream::from(derive_ui)
}


/**
The `NwgPartial` macro implements the native-windows-gui `PartialUi` trait on the selected struct

`NwgPartial` accepts the same attributes as `NwgUi`. See the docs of the `NwgUi` trait for detailed usage. There are some particularities though:

 - Partials cannot be used by independently. They must be included in a UI that implements `NwgUi`. 
 - Partials do not require a top level window. If no window is defined, the partial will require a parent value passed from the `nwg_partial` attribute
 - It's possible to derive both `NwgUi` and `NwgPartial` from the same struct as long as the partial do not need a parent.
 - Partials can contains other partials

```
#[derive(Default, NwgPartial)]
pub struct MyPartial {
  partial_data: u32,

  #[nwg_control]
  button: nwg::Button
}

#[derive(Default, NwgUi)]
pub struct MyApp {
   app_data: u32,

   #[nwg_control]
   #[nwg_events( OnInit: [hello], OnWindowClose: [nwg::stop_thread_dispatch()] )]
   window: nwg::Window,

   #[nwg_partial(parent: window)]
   partial: MyPartial
}
```

*/
#[proc_macro_derive(NwgPartial, attributes(nwg_control, nwg_resource, nwg_events, nwg_layout, nwg_layout_item, nwg_partial))]
pub fn derive_partial(input: pm::TokenStream) -> pm::TokenStream {
    let base = parse_macro_input!(input as DeriveInput);

    let names = parse_base_names(&base);

    let partial_name = &names.n_partial_module;
    let struct_name = &names.n_struct;

    let lt = &base.generics.lt_token;
    let generic_params = &base.generics.params;
    let generic_names = extract_generic_names(generic_params);
    let gt = &base.generics.gt_token;
    let where_clause = &base.generics.where_clause;

    let generics = quote! { #lt #generic_params #gt }; // <'a: 'b, T: Trait1, const C>
    let generic_names = quote! { #lt #generic_names #gt }; // <'a, T, C>

    let ui_data = parse_ui_data(&base).expect("NWG derive can only be implemented on structs");
    let ui = NwgUi::build(&ui_data, true);
    let controls = ui.controls();
    let resources = ui.resources();
    let partials = ui.partials();
    let layouts = ui.layouts();
    let events = ui.events();

    let nwg_name = crate_name("native-windows-gui");
    
    // Returns an error in the examples, so we try a default value
    let nwg = match nwg_name {
        Ok(name) => syn::Ident::new(&name, proc_macro2::Span::call_site()),
        Err(_) => syn::Ident::new("native_windows_gui", proc_macro2::Span::call_site()),   
    };

    let partial_ui = quote! {
        mod #partial_name {
            extern crate #nwg as nwg;
            use nwg::*;
            use super::*;
        
            impl #generics PartialUi for #struct_name #generic_names #where_clause {

                #[allow(unused)]
                fn build_partial<W: Into<ControlHandle>>(data: &mut Self, _parent: Option<W>) -> Result<(), NwgError> {
                    let parent = _parent.map(|p| p.into());
                    let parent_ref = parent.as_ref();
                    
                    #resources
                    #controls
                    #partials

                    let ui = data;
                    #layouts
                    Ok(())
                }

                fn process_event<'a>(&self, _evt: Event, _evt_data: &EventData, _handle: ControlHandle) {
                    #events
                }

                fn handles(&self) -> Vec<&ControlHandle> {
                    Vec::new()
                }
            }
        }
    };

    pm::TokenStream::from(partial_ui)
}
