/*!
    How to use an external rendering API with the NWG ExternalCanvas.
    Also show how NWG controls can be subclassed

    Requires the following features: `cargo run --example opengl_canvas --features "color-dialog extern-canvas"`
*/
extern crate glutin;
extern crate gl;
#[macro_use] extern crate native_windows_gui as nwg;

use std::cell::RefCell;
use crate::glutin::{
    ContextBuilder, GlRequest, GlProfile, PossiblyCurrent, RawContext, Api,
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

You can either to it manually or the `subclass_control!(type, base_type, field)` macro.
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
        use std::{mem, ptr};
        
        unsafe {
            let ctx = ContextBuilder::new()
                .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
                .with_gl_profile(GlProfile::Core)
                .build_raw_context(self.canvas.handle.hwnd().unwrap() as *mut c_void)
                .expect("Failed to build opengl context")
                .make_current()
                .expect("Failed to set opengl context as current");
        
            // Load the function pointers
            gl::Clear::load_with(|s| ctx.get_proc_address(s) as *const _);
            gl::ClearColor::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::CreateShader::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::ShaderSource::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::CompileShader::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::CreateProgram::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::AttachShader::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::LinkProgram::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::UseProgram::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::GenBuffers::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::BindBuffer::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::BufferData::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::GetAttribLocation::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::VertexAttribPointer::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::EnableVertexAttribArray::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::GenVertexArrays::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::BindVertexArray::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::DrawArrays::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::Viewport::load_with(|s| ctx.get_proc_address(s) as *const _ );
            gl::BufferSubData::load_with(|s| ctx.get_proc_address(s) as *const _ );

            // Init default state
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);

            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vs, 1, [VS_SRC.as_ptr() as *const _].as_ptr(), ptr::null());
            gl::CompileShader(vs);

            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fs, 1, [FS_SRC.as_ptr() as *const _].as_ptr(), ptr::null());
            gl::CompileShader(fs);

            let program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);
            gl::UseProgram(program);

            let vertex_data: &[f32] = &[
                0.0,  1.0,   1.0, 1.0, 1.0,
               -1.0, -1.0,   1.0, 1.0, 1.0,
                1.0, -1.0,   1.0, 1.0, 1.0,
            ];
            let vertex_size = vertex_data.len() * mem::size_of::<f32>();

            let mut vb = mem::zeroed();
            gl::GenBuffers(1, &mut vb);
            gl::BindBuffer(gl::ARRAY_BUFFER, vb);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                vertex_size as gl::types::GLsizeiptr,
                vertex_data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let mut vao = mem::zeroed();
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);

            let stride = mem::size_of::<f32>() * 5;
            let color_offset = 8 as *const c_void; 
            gl::VertexAttribPointer(0, 2, gl::FLOAT, 0, stride as i32, ptr::null());
            gl::VertexAttribPointer(1, 4, gl::FLOAT, 0, stride as i32, color_offset);
            
           
            *self.ctx.borrow_mut() = Some(ctx);
        }
    }

    /// Our render function
    pub fn render(&self) {
        self.ctx.borrow().as_ref().map(|ctx| unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            ctx.swap_buffers().unwrap();
        });
    }

    pub fn resize(&self) {
        self.ctx.borrow().as_ref().map(|ctx| unsafe {
            let (w, h) = self.canvas.physical_size();
            gl::Viewport(0, 0, w as _, h as _);
            ctx.resize(PhysicalSize::new(w as f64, h as f64));
        });

        self.render();
    }

}

subclass_control!(OpenGlCanvas, ExternCanvas, canvas);

// subclass_control generates the following code

/*
use std::ops::{Deref, DerefMut};
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
}*/



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

    pub fn show(&self) {
        self.window.set_visible(true);
        self.window.set_focus();
    }

    pub fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    pub fn resize_canvas(&self) {
        self.canvas.resize();
    }

    pub fn select_bg_color(&self) {
        if self.color_dialog.run(Some(&self.window)) {
            let [r, g, b] = self.color_dialog.color();
            let [r, g, b] = [r as f32 / 225.0, g as f32 / 225.0, b as f32 / 225.0];

            unsafe {
                gl::ClearColor(r, g, b, 1.0);
            }
        }

        self.window.invalidate();
    }
    
    pub fn select_tri_color(&self) {
        use std::mem;

        if self.color_dialog.run(Some(&self.window)) {
            let [r, g, b] = self.color_dialog.color();
            let [r, g, b] = [r as f32 / 225.0, g as f32 / 225.0, b as f32 / 225.0];

            let vertex_data: &[f32] = &[
                0.0,  1.0,   r, g, b,
               -1.0, -1.0,   r, g, b,
                1.0, -1.0,   r, g, b,
            ];

            let vertex_size = vertex_data.len() * mem::size_of::<f32>();

            unsafe {
                gl::BufferSubData(gl::ARRAY_BUFFER, 0, vertex_size as gl::types::GLsizeiptr, vertex_data.as_ptr() as *const _);
            }
        }

        self.window.invalidate();
    }

}


mod extern_canvas_ui {
    use native_windows_gui as nwg;
    use super::*;
    use std::rc::Rc;
    use std::ops::Deref;

    pub struct ExternCanvasUi {
        inner: Rc<ExternCanvas>,
        default_handler: RefCell<Vec<nwg::EventHandler>>
    }

    impl nwg::NativeUi<ExternCanvasUi> for ExternCanvas {
        fn build_ui(mut data: ExternCanvas) -> Result<ExternCanvasUi, nwg::NwgError> {
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
            let ui = ExternCanvasUi {
                inner: Rc::new(data),
                default_handler: RefCell::default(),
            };

            // Events
            let window_handles = [&ui.window.handle];

            for handle in window_handles.iter() {
                let evt_ui = Rc::downgrade(&ui.inner);
                let handle_events = move |evt, _evt_data, handle| {
                    if let Some(evt_ui) = evt_ui.upgrade() {
                        match evt {
                            E::OnResize => {
                                if &handle == &evt_ui.canvas {
                                    ExternCanvas::resize_canvas(&evt_ui);
                                }
                            },
                            E::OnButtonClick => {
                                if &handle == &evt_ui.choose_color_btn1 {
                                    ExternCanvas::select_bg_color(&evt_ui);
                                } else if &handle == &evt_ui.choose_color_btn2 {
                                    ExternCanvas::select_tri_color(&evt_ui);
                                }
                            },
                            E::OnWindowClose => {
                                if &handle == &evt_ui.window {
                                    ExternCanvas::exit(&evt_ui);
                                }
                            },
                            E::OnInit => {
                                if &handle == &evt_ui.window {
                                    ExternCanvas::show(&evt_ui);
                                }
                            },
                            _ => {}
                        }
                    }
                };

                ui.default_handler.borrow_mut().push(
                    nwg::full_bind_event_handler(handle, handle_events)
                );
            }

            // Layouts
            nwg::GridLayout::builder()
                .parent(&ui.window)
                .max_column(Some(4))
                .max_row(Some(8))
                .child_item(nwg::GridLayoutItem::new(&ui.canvas, 0, 0, 3, 8))
                .child(3, 0, &ui.choose_color_btn1)
                .child(3, 1, &ui.choose_color_btn2)
                .build(&ui.layout)?;
            
            return Ok(ui);
        }
    }

    impl Drop for ExternCanvasUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let mut handlers = self.default_handler.borrow_mut();
            for handler in handlers.drain(0..) {
                nwg::unbind_event_handler(&handler);
            }
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

    // Here we use the `with_callback` version of dispatch_thread_events
    // Internally the callback will be executed almost as fast as `loop { callback() }`
    nwg::dispatch_thread_events_with_callback(move || {
        app.canvas.render();
    });
}


const VS_SRC: &'static [u8] = b"#version 330
layout (location=0) in vec2 a_position;
layout (location=1) in vec4 a_color;

out vec4 color;

void main() {
    color = a_color;
    gl_Position = vec4(a_position, 0.0, 1.0);
}
\0";

const FS_SRC: &'static [u8] = b"#version 330
precision mediump float;

in vec4 color;

out vec4 outColor;
 
void main() {
    outColor = color;
}
\0";
