# Native Windows GUI

Welcome to Native Windows GUI (aka NWG). The BEST (and only) rust library to develop truly native GUI applications on desktop for Microsoft Windows.

NWG is a very light wrapper over WINAPI. It allows you, the developer, to handle
the quirks and rough edges of the API by providing a simple, safe and rust-like interface.

Due to its simple nature NWG is sure to keep your compile time low, your resources
usage at a minimum and your mind somewhat "sane".

This is the 3rd and final version of NWG. It is considered "mature" or, as I would say
"the backlog is empty, and it will most likely stay that way". This version implements pretty much
everything required to develop applications on Windows. Don't bother using the older versions as they
have "irreconcilable design decisions" and cannot support some key features.

If you've managed to read though this introduction, you should know that my twitter handle
is [#gdube_dev](https://twitter.com/gdube_dev) and you can support this project with *GitHub Sponsors*.
Any support is greatly appreciated.

## Project structure

This is the main project git. It is separated in multiple sections

- native-windows-gui
  - The base library. Includes an interactive test suite and plenty of examples
- native-windows-derive
  - A procedural macro that generates GUI application from rust structure (pretty cool stuff IMO)
- native-windows-docs
  - A hefty documentation that goes over everything you need to know about NWG
- linda-cat-cafe
  - The final result of the "cat cafe" tutorial in the docs
- [showcase](https://github.com/gabdube/nwg-private/tree/master/showcase)
  - Images of the examples. If you've made a NWG application and want
  to share it here. Send me a message or open a PR. It's free real estate.

## Supported features

* The WHOLE winapi control library [(reference)](https://docs.microsoft.com/en-us/windows/win32/controls/individual-control-info)
  * Some very niche controls are not supported: flat scroll bar, ip control, rebar, and pager.
 * Menus and menu bar 
 * Image and font resource
 * Tooltip and system tray notification
 * Partial templates support
   * Split large application into chunks
 * Dynamic controls support
   * Add/Remove controls at runtime
   * Bind or unbind new events at runtime
 * Multithreaded application support
   * Communicate to the GUI thread from another thread
   * Run multiple window on different threads
 * Simple layout configurations
    * HBoxLayout
    * VBoxLayout
    * GridLayout
 * A canvas powered by *Direct2D* to draw custom controls
 * Extended image formats with the Windows Imaging Component (WIC).
 * The most common dialog boxes
   * File dialog (save, open, open folder)
   * Font dialog
   * Color dialog
 * Support fow low level system message capture. 
 * Cross compiling and testing from Linux to Windows with Wine and mingw.

## Performance

This was measured on a `Intel(R) Core(TM) i7-3770 CPU @ 3.40GHz, 3401 Mhz, 4 Core(s), 8 Logical Processor(s)`

In release mode, the `basic` example weight **163kb** on disk and take **900kb** in memory. Launch time is instantaneous.

The interactive test suit (with over 50 controls) weight **671 kb** on disk and take **1100kb** in memory. Launch time is still instantaneous.

Initial build time takes around **22 seconds** for a basic applications. This is mainly due to `winapi-rs` taking its sweet time. Subsequent compile time takes around **0.7 seconds**.

## Development

The development of this library is considered "done". By that, I mean that
there won't by any change to the API. Issues can be raised if a bug is found or
if some area in the documentation are unclear. If I overlooked a very important feature,
it might also be added.

## Code example

### With native windows derive

```rust
TODO
```

### Without native windows derive

```rust
#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
use nwg::NativeUi;


#[derive(Default)]
pub struct BasicApp {
    window: nwg::Window,
    name_edit: nwg::TextInput,
    hello_button: nwg::Button
}

impl BasicApp {

    fn say_hello(&self, _event: nwg::Event) {
        nwg::simple_message("Hello", &format!("Hello {}", self.name_edit.text()));
    }

    fn say_goodbye(&self, _event: nwg::Event) {
        nwg::simple_message("Goodbye", &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();
    }

}

//
// ALL of this stuff is handled by native-windows-derive
//
mod basic_app_ui {
    use native_windows_gui as nwg;
    use super::*;
    use std::rc::Rc;
    use std::ops::Deref;

    pub struct BasicAppUi {
        inner: BasicApp
    }

    impl nwg::NativeUi<BasicApp, BasicAppUi> for BasicApp {
        fn build_ui(mut data: BasicApp) -> Result<Rc<BasicAppUi>, nwg::SystemError> {
            use nwg::Event as E;
            
            // Controls
            nwg::Window::builder()
                .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
                .size((300, 115))
                .position((300, 300))
                .title("Basic example")
                .build(&mut data.window)?;

            nwg::TextInput::builder()
                .size((280, 25))
                .position((10, 10))
                .text("Heisenberg")
                .parent(&data.window)
                .build(&mut data.name_edit)?;

            nwg::Button::builder()
                .size((280, 60))
                .position((10, 40))
                .text("Say my name")
                .parent(&data.window)
                .build(&mut data.hello_button)?;

            // Wrap-up
            let ui = Rc::new(BasicAppUi { inner: data });

            // Events
            let window_handles = [&ui.window.handle];
            for handle in window_handles.iter() {
                let evt_ui = ui.clone();
                let handle_events = move |evt, handle| {
                    match evt {
                        E::OnButtonClick => {
                            if handle == evt_ui.hello_button.handle {
                                BasicApp::say_hello(&evt_ui.inner, evt);
                            }
                        },
                        E::OnWindowClose => {
                            if handle == evt_ui.window.handle {
                                BasicApp::say_goodbye(&evt_ui.inner, evt);
                            }
                        },
                        _ => {}
                    }
                };

                nwg::bind_event_handler(handle, handle_events);
            }
            
            return Ok(ui);
        }
    }


    impl Deref for BasicAppUi {
        type Target = BasicApp;

        fn deref(&self) -> &BasicApp {
            &self.inner
        }
    }

}



fn main() {
    nwg::enable_visual_styles();
    nwg::init_common_controls().expect("Failed to init common controls");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
```
