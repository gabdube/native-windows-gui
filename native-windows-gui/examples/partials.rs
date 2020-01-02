/*!
    An application that load different interfaces using the partial feature.
    Partials can be used to split large GUI application into smaller bits.

    Requires the following features: `cargo run --example partials --features "listbox frame"`
*/

extern crate native_windows_gui as nwg;
use nwg::NativeUi;


#[derive(Default)]
pub struct PartialDemo {
    window: nwg::Window,
    layout: nwg::BoxLayout,
    menu: nwg::ListBox<&'static str>,
    frame1: nwg::Frame,
    frame2: nwg::Frame,
    frame3: nwg::Frame
}

impl PartialDemo {
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

//
// ALL of this stuff is handled by native-windows-derive
//
mod partial_demo_ui {
    use native_windows_gui as nwg;
    use super::*;
    use std::rc::Rc;
    use std::ops::Deref;

    pub struct PartialDemoUi {
        inner: PartialDemo
    }

    impl nwg::NativeUi<PartialDemo, PartialDemoUi> for PartialDemo {
        fn build_ui(mut data: PartialDemo) -> Result<Rc<PartialDemoUi>, nwg::NwgError> {
            use nwg::Event as E;
            
            // Controls
            nwg::Window::builder()
                .size((500, 400))
                .position((300, 300))
                .title("Many UI")
                .build(&mut data.window)?;

            nwg::ListBox::builder()
                .collection(vec!["People", "Animals", "Food"])
                .parent(&data.window)
                .build(&mut data.menu)?;

            nwg::Frame::builder()
                .parent(&data.window)
                .build(&mut data.frame1)?;

            nwg::Frame::builder()
                .flags(nwg::FrameFlags::BORDER)
                .parent(&data.window)
                .build(&mut data.frame2)?;

            nwg::Frame::builder()
                .flags(nwg::FrameFlags::BORDER)
                .parent(&data.window)
                .build(&mut data.frame3)?;

            // Wrap-up
            let ui = Rc::new(PartialDemoUi { inner: data });

            // Events
            let evt_ui = ui.clone();
            let handle_events = move |evt, _evt_data, handle| {
                match evt {
                    E::OnWindowClose => {
                        if &handle == &evt_ui.window {
                            PartialDemo::exit(&evt_ui.inner);
                        }
                    },
                    _ => {}
                }
            };

            nwg::full_bind_event_handler(&ui.window.handle, handle_events);

            // Layout

            nwg::BoxLayout::builder()
                .parent(&ui.window)
                .layout_type(nwg::BoxLayoutType::Horizontal)
                .cell_count(Some(4))
                .child(0, &ui.menu)
                .child_item(nwg::BoxLayoutItem::new(&ui.frame1, 1, 3))
                .child_item(nwg::BoxLayoutItem::new(&ui.frame2, 1, 3))
                .child_item(nwg::BoxLayoutItem::new(&ui.frame3, 1, 3))
                .build(&ui.layout);
            
            return Ok(ui);
        }
    }


    impl Deref for PartialDemoUi {
        type Target = PartialDemo;

        fn deref(&self) -> &PartialDemo {
            &self.inner
        }
    }

}



fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _ui = PartialDemo::build_ui(Default::default()).expect("Failed to build UI");
    
    nwg::dispatch_thread_events();
}
