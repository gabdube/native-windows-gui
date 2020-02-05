/*!
    An application that runs in the system tray.

    Requires the following features: `cargo run --example system_tray --features "tray-notification message-window menu cursor"`
*/
extern crate native_windows_gui as nwg;
use nwg::NativeUi;


#[derive(Default)]
pub struct SystemTray {
    window: nwg::MessageWindow,
    icon: nwg::Icon,
    tray: nwg::TrayNotification,
    tray_menu: nwg::Menu,
    tray_item1: nwg::MenuItem,
    tray_item2: nwg::MenuItem,
    tray_item3: nwg::MenuItem,
}

impl SystemTray {

    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn hello1(&self) {
        nwg::simple_message("Hello", "Hello World!");
    }
    
    fn hello2(&self) {
        let flags = nwg::TrayNotificationFlags::USER_ICON | nwg::TrayNotificationFlags::LARGE_ICON;
        self.tray.show("Hello World", Some("Welcome to my application"), Some(flags), Some(&self.icon));
    }
    
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}


//
// ALL of this stuff is handled by native-windows-derive
//
mod system_tray_ui {
    use native_windows_gui as nwg;
    use super::*;
    use std::rc::Rc;
    use std::ops::Deref;

    pub struct SystemTrayUi {
        inner: SystemTray
    }

    impl nwg::NativeUi<SystemTray, Rc<SystemTrayUi>> for SystemTray {
        fn build_ui(mut data: SystemTray) -> Result<Rc<SystemTrayUi>, nwg::NwgError> {
            use nwg::Event as E;

            // Resources
            nwg::Icon::builder()
                .source_file(Some("./test_rc/cog.ico"))
                .build(&mut data.icon)?;
            
            // Controls
            nwg::MessageWindow::builder()
                .build(&mut data.window)?;

            nwg::TrayNotification::builder()
                .parent(&data.window)
                .icon(Some(&data.icon))
                .tip(Some("Hello"))
                .build(&mut data.tray)?;

            nwg::Menu::builder()
                .popup(true)
                .parent(&data.window)
                .build(&mut data.tray_menu)?;

            nwg::MenuItem::builder()
                .text("Hello")
                .parent(&data.tray_menu)
                .build(&mut data.tray_item1)?;

            nwg::MenuItem::builder()
                .text("Popup")
                .parent(&data.tray_menu)
                .build(&mut data.tray_item2)?;

            nwg::MenuItem::builder()
                .text("Exit")
                .parent(&data.tray_menu)
                .build(&mut data.tray_item3)?;

            // Wrap-up
            let ui = Rc::new(SystemTrayUi { inner: data });

            // Events
            let evt_ui = ui.clone();
            let handle_events = move |evt, _evt_data, handle| {
                match evt {
                    E::OnContextMenu => 
                        if &handle == &evt_ui.tray {
                            SystemTray::show_menu(&evt_ui.inner);
                        }
                    E::OnMenuItemSelected => 
                        if &handle == &evt_ui.tray_item1 {
                            SystemTray::hello1(&evt_ui.inner);
                        } else if &handle == &evt_ui.tray_item2 {
                            SystemTray::hello2(&evt_ui.inner);
                        } else if &handle == &evt_ui.tray_item3 {
                            SystemTray::exit(&evt_ui.inner);
                        },
                    _ => {}
                }
            };

            nwg::full_bind_event_handler(&ui.window.handle, handle_events);

            return Ok(ui);
        }
    }

    impl Deref for SystemTrayUi {
        type Target = SystemTray;

        fn deref(&self) -> &SystemTray {
            &self.inner
        }
    }

}


fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _ui = SystemTray::build_ui(Default::default()).expect("Failed to build UI");
    
    nwg::dispatch_thread_events();
}
