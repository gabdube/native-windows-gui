/*!
    How to use an external rendering API with the NWG ExternalCanvas.
    Also show how NWG controls can be subclassed

    Requires the follwing features: `cargo run example opengl_canvas --features "color-dialog extern-canvas"`
*/
extern crate glutin;
extern crate gl;
extern crate native_windows_gui as nwg;

use std::{
    cell::RefCell,
    ops::{Deref, DerefMut}
};
use crate::glutin::{
    ContextBuilder, GlRequest, GlProfile, PossiblyCurrent, RawContext,
    dpi::PhysicalSize,
    os::windows::RawContextExt
};
use crate::nwg::NativeUi;


type Ctx = RawContext<PossiblyCurrent>;

/**
Specialize the canvas.

To register a custom struct as a NWG control with full support you need to implement 4 traits:
  * Deref
  * DerefMut
  * Into<nwg::ControlHandle>
  * PartialEq<SubclassControl> for nwg::ControlHandle

You can either to it manually or the `register_control(type, base_type, field)` macro.
*/
#[derive(Default)]
pub struct OpenGlCanvas {
    ctx: RefCell<Option<Ctx>>,
    canvas: nwg::ExternCanvas,
}

impl OpenGlCanvas {

    /// Create an opengl canvas with glutin & gl
    pub fn create_context(&self) {
        use std::ffi::c_void;
        
        unsafe {
            let ctx = ContextBuilder::new()
                .with_gl(GlRequest::Latest)
                .with_gl_profile(GlProfile::Core)
                .build_raw_context(self.canvas.handle.hwnd().unwrap() as *mut c_void)
                .expect("Failed to build opengl context")
                .make_current()
                .expect("Failed to set opengl context as current");
        
            // Load the function pointers
            gl::Clear::load_with(|s| ctx.get_proc_address(s) as *const _);
            gl::ClearColor::load_with(|s| ctx.get_proc_address(s) as *const _ );

            // Init default state
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);

            *self.ctx.borrow_mut() = Some(ctx);
        }
    }

    /// Our render function
    pub fn render(&self) {
        self.ctx.borrow().as_ref().map(|ctx| unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            ctx.swap_buffers().unwrap();
        });
    }

    pub fn resize(&self) {
        self.ctx.borrow().as_ref().map(|ctx| {
            let (w, h) = self.canvas.size();
            ctx.resize(PhysicalSize::new(w as f64, h as f64));
        });

        self.render();
    }

}

impl Deref for OpenGlCanvas {
    type Target = nwg::ExternCanvas;
    fn deref(&self) -> &nwg::ExternCanvas { &self.canvas }
}

impl DerefMut for OpenGlCanvas {
    fn deref_mut(&mut self) -> &mut Self::Target {&mut self.canvas }
}

impl Into<nwg::ControlHandle> for &OpenGlCanvas {
    fn into(self) -> nwg::ControlHandle { self.canvas.handle.clone() }
}

impl PartialEq<OpenGlCanvas> for nwg::ControlHandle {
    fn eq(&self, other: &OpenGlCanvas) -> bool {
        *self == other.handle
    }
}



/**
    The Ui application. Spoiler alert, there's nothing much different from the other examples.
*/
#[derive(Default)]
pub struct ExternCanvas {
    window: nwg::Window,
    layout: nwg::GridLayout,
    canvas: OpenGlCanvas,
    color_dialog: nwg::ColorDialog,
    choose_color_btn1: nwg::Button,
    choose_color_btn2: nwg::Button,
}

impl ExternCanvas {

    pub fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    pub fn resize_canvas(&self) {
        self.canvas.resize();
    }

    pub fn select_bg_color(&self) {
        if self.color_dialog.show(Some(&self.window)) {
            let [r, g, b] = self.color_dialog.color();
            unsafe {
                gl::ClearColor(f32::from(r) / 255.0, f32::from(g) / 255.0, f32::from(b) / 255.0, 1.0);
            }
        }
    }
    
    pub fn select_tri_color(&self) {
        if self.color_dialog.show(Some(&self.window)) {
            //let [r, g, b] = self.color_dialog.color();
        }
    }

}


mod extern_canvas_ui {
    use native_windows_gui as nwg;
    use super::*;
    use std::rc::Rc;
    use std::ops::Deref;

    pub struct ExternCanvasUi {
        inner: ExternCanvas
    }

    impl nwg::NativeUi<ExternCanvas, ExternCanvasUi> for ExternCanvas {
        fn build_ui(mut data: ExternCanvas) -> Result<Rc<ExternCanvasUi>, nwg::NwgError> {
            use nwg::Event as E;
            
            // Resources
            nwg::ColorDialog::builder()
                .build(&mut data.color_dialog)?;

            // Controls
            nwg::Window::builder()
                .flags(nwg::WindowFlags::MAIN_WINDOW)
                .size((600, 500))
                .position((300, 300))
                .title("Native windows GUI / OpenGL")
                .build(&mut data.window)?;

            nwg::ExternCanvas::builder()
                .parent(Some(&data.window))
                .build(&mut data.canvas)?;

            nwg::Button::builder()
                .text("Background color")
                .parent(&data.window)
                .build(&mut data.choose_color_btn1)?;

            nwg::Button::builder()
                .text("Triangle color")
                .parent(&data.window)
                .build(&mut data.choose_color_btn2)?;

            // Wrap-up
            let ui = Rc::new(ExternCanvasUi { inner: data });

            // Events
            let window_handles = [&ui.window.handle];

            for handle in window_handles.iter() {
                let evt_ui = ui.clone();
                let handle_events = move |evt, _evt_data, handle| {
                    match evt {
                        E::OnResize => {
                            if &handle == &evt_ui.canvas {
                                ExternCanvas::resize_canvas(&evt_ui.inner);
                            }
                        },
                        E::OnButtonClick => {
                            if &handle == &evt_ui.choose_color_btn1 {
                                ExternCanvas::select_bg_color(&evt_ui.inner);
                            } else if &handle == &evt_ui.choose_color_btn2 {
                                ExternCanvas::select_tri_color(&evt_ui.inner);
                            }
                        },
                        E::OnWindowClose => {
                            if &handle == &evt_ui.window {
                                ExternCanvas::exit(&evt_ui.inner);
                            }
                        },
                        _ => {}
                    }
                };

                nwg::full_bind_event_handler(handle, handle_events);
            }

            // Layouts
            nwg::GridLayout::builder()
                .parent(&ui.window)
                .max_column(Some(4))
                .max_row(Some(8))
                .child_item(nwg::GridLayoutItem::new(&ui.canvas, 0, 0, 3, 8))
                .child(3, 0, &ui.choose_color_btn1)
                .child(3, 1, &ui.choose_color_btn2)
                .build(&ui.layout);
            
            return Ok(ui);
        }
    }


    impl Deref for ExternCanvasUi {
        type Target = ExternCanvas;

        fn deref(&self) -> &ExternCanvas {
            &self.inner
        }
    }

}

pub fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let app = ExternCanvas::build_ui(Default::default()).expect("Failed to build UI");

    // Make sure to render everything at least once before showing the window to remove weird artifacts.
    app.canvas.create_context();
    app.canvas.render();
    app.window.set_visible(true);
    app.window.set_focus();

    // Here we use the `with_callback` version of dispatch_thread_events
    // Internally the callback will be executed almost as fast as `loop { callback() }`
    nwg::dispatch_thread_events_with_callback(move || {
        app.canvas.render();
    });
}
