/**
    A very simple application that show your name in a message box.

    This demo shows how to use NWG without the NativeUi trait boilerplate.
    Note that this way of doing things is alot less extensible and cannot make use of native windows derive.

    See `basic` for the NativeUi version and `basic_d` for the derive version
*/
extern crate native_windows_gui as nwg;
use std::rc::Rc;

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let mut window = Default::default();
    let mut name_edit = Default::default();
    let mut hello_button = Default::default();
    let layout = Default::default();

    nwg::Window::builder()
        .size((300, 115))
        .position((300, 300))
        .title("Basic example")
        .build(&mut window)
        .unwrap();

    nwg::TextInput::builder()
        .text("Heisenberg")
        .focus(true)
        .parent(&window)
        .build(&mut name_edit)
        .unwrap();

    nwg::Button::builder()
        .text("Say my name")
        .parent(&window)
        .build(&mut hello_button)
        .unwrap();

    nwg::GridLayout::builder()
        .parent(&window)
        .spacing(1)
        .child(0, 0, &name_edit)
        .child_item(nwg::GridLayoutItem::new(&hello_button, 0, 1, 1, 2))
        .build(&layout)
        .unwrap();

    let window = Rc::new(window);
    let events_window = window.clone();

    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
        use nwg::Event as E;

        match evt {
            E::OnWindowClose => 
                if &handle == &events_window as &nwg::Window {
                    nwg::modal_info_message(&events_window.handle, "Goodbye", &format!("Goodbye {}", name_edit.text()));
                    nwg::stop_thread_dispatch();
                },
            E::OnButtonClick => 
                if &handle == &hello_button {
                    nwg::modal_info_message(&events_window.handle, "Hello", &format!("Hello {}", name_edit.text()));
                },
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}
