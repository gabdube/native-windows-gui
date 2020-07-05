extern crate native_windows_gui as nwg;

use nwg::{NativeUi, PartialUi};
use std::{rc::Rc, cell::RefCell, ops::Deref};

#[derive(Default)]
pub struct MainUi {
    window: nwg::Window,
    form: SubmitForm,
}

#[derive(Default)]
pub struct SubmitForm {
    form_data: String,
    layout: nwg::GridLayout,
    value: nwg::TextInput,
    sumbit_button: nwg::Button
}

pub struct MainUiWrapper {
    inner: Rc<MainUi>,
    default_handler: RefCell<Vec<nwg::EventHandler>>
}

impl nwg::NativeUi<MainUiWrapper> for MainUi {
    fn build_ui(mut data: MainUi) -> Result<MainUiWrapper, nwg::NwgError> {
        nwg::Window::builder()
            .size((500, 200))
            .position((500, 300))
            .title("My Form")
            .build(&mut data.window)?;

        // !!! Partials controls setup !!!
        SubmitForm::build_partial(&mut data.form, Some(&data.window))?;

        let ui = MainUiWrapper {
            inner: Rc::new(data),
            default_handler: Default::default(),
        };

        // !!! Partials Event Binding !!!
        let mut window_handles = vec![&ui.window.handle];
        window_handles.append(&mut ui.form.handles());

        for handle in window_handles.iter() {
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, evt_data, handle| {
                use nwg::Event as E;

                if let Some(ui) = evt_ui.upgrade() {

                    // !!! Partials Event Dispatch !!!
                    ui.form.process_event(evt, &evt_data, handle);

                    match evt {
                        E::OnButtonClick => 
                            if &handle == &ui.form.sumbit_button {
                                println!("SAVING!");
                            },
                        E::OnWindowClose => 
                            if &handle == &ui.window {
                                nwg::stop_thread_dispatch();
                            },
                        _ => {}
                    }
                }
            };

            ui.default_handler.borrow_mut().push(
                nwg::full_bind_event_handler(handle, handle_events)
            );
        }

        return Ok(ui);
    }
}

impl Deref for MainUiWrapper {
    type Target = MainUi;

    fn deref(&self) -> &MainUi {
        &self.inner
    }
}


impl PartialUi for SubmitForm {

    fn build_partial<W: Into<nwg::ControlHandle>>(data: &mut SubmitForm, parent: Option<W>) -> Result<(), nwg::NwgError> {
        let parent = parent.unwrap().into();

        nwg::TextInput::builder()
            .text(&data.form_data)
            .parent(&parent)
            .build(&mut data.value)?;

        nwg::Button::builder()
            .text("Save")
            .parent(&parent)
            .build(&mut data.sumbit_button)?;

        nwg::GridLayout::builder()
            .child(0, 0, &data.value)
            .child(0, 1, &data.sumbit_button)
            .parent(&parent)
            .build(&data.layout)?;

        Ok(())
    }

    fn process_event<'a>(&self, evt: nwg::Event, _evt_data: &nwg::EventData, handle: nwg::ControlHandle) {
        use nwg::Event as E;

        match evt {
            E::OnButtonClick => 
                if &handle == &self.sumbit_button {
                    println!("PARTIAL EVENT!");
                },
            _ => {}
        }
    }

    fn handles(&self) -> Vec<&nwg::ControlHandle> {
        // No top level window in this partial
        Vec::new()
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let state = MainUi {
        form: SubmitForm {
            form_data: "Default Value".to_string(),
            ..Default::default()
        },
        ..Default::default()
    };

    let _ui = MainUi::build_ui(state).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}

