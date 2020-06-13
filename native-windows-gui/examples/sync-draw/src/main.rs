/*!
    An application that lets you draw onto a canvas. If multiple application are running they all share the canvas.

    This is an example of a medium sized NWG application.
*/

// Disable this to show the console
#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;
extern crate glutin;
extern crate gl;

mod win32_event;
use win32_event::{Win32Event, Win32EventWaitResult};

mod opengl_canvas;
use opengl_canvas::OpenGlCanvas;

mod shared_memory;
use shared_memory::{SharedMemory};

mod data;
use data::{AppData, AppMode};

use nwd::NwgUi;
use nwg::NativeUi;

use std::cell::RefCell;


/**
    Base of the main application. Regroups the UI definition and the application data.
*/
#[derive(Default, NwgUi)]
pub struct SyncDraw {

    // Application data
    data: RefCell<AppData>,

    // Instances update notice
    #[nwg_control(parent: window)]
    #[nwg_events(OnNotice: [SyncDraw::refresh])]
    notice: nwg::Notice,

    // Window and layout
    #[nwg_control(size: (400, 300), position: (700, 150), title: "SyncDraw", icon: Some(&data.pen_icon), flags: "MAIN_WINDOW" )]
    #[nwg_events( 
        OnWindowClose: [SyncDraw::exit], OnInit: [SyncDraw::setup],
        OnResize: [SyncDraw::update_size], OnResizeEnd: [SyncDraw::update_size], OnWindowMaximize: [SyncDraw::update_size]
    )]
    window: nwg::Window,

    #[nwg_layout(parent: window, margin: [40, 5, 30, 5])]
    layout: nwg::GridLayout,

    // Resources
    #[nwg_resource(source_file: Some("./pencil.ico"), size: Some((16, 16)), strict: true )]
    pen_icon: nwg::Icon,

    #[nwg_resource(source_file: Some("./eraser.ico"), size: Some((16, 16)), strict: true )]
    eraser_icon: nwg::Icon,

    #[nwg_resource(family: "Arial", size: 17)]
    font: nwg::Font,

    // Refresh timer (30 FPS)
    #[nwg_control(parent: window, interval: 33, stopped: false)]
    #[nwg_events(OnTimerTick: [SyncDraw::render])]
    refresh_timer: nwg::Timer,

    // Static Buttons
    #[nwg_control(text: "Mode:", position: (10, 10), size: (50, 10), font: Some(&data.font) )]
    mode_lbl: nwg::Label,

    #[nwg_control(text: " Draw", position: (60, 10), icon: Some(&data.pen_icon), size: (100, 30), font: Some(&data.font) )]
    #[nwg_events(OnButtonClick: [SyncDraw::update_mode(SELF, CTRL)] )]
    pencil_btn: nwg::Button,

    #[nwg_control(text: " Erase", position: (170, 10), size: (100, 30), icon: Some(&data.eraser_icon), font: Some(&data.font) )]
    #[nwg_events(OnButtonClick: [SyncDraw::update_mode(SELF, CTRL)] )]
    eraser_btn: nwg::Button,

    // Canvas
    #[nwg_control(ty: ExternCanvas, parent: Some(&data.window) )]
    #[nwg_layout_item(layout: layout, cell: 0)]
    #[nwg_events(OnMouseMove: [SyncDraw::paint], MousePressLeftUp: [SyncDraw::update_draw(SELF, EVT)], MousePressLeftDown: [SyncDraw::update_draw(SELF, EVT)])]
    canvas: OpenGlCanvas,

    // Status bar
    #[nwg_control(parent: window, font: Some(&data.font))]
    status: nwg::StatusBar
}

impl SyncDraw {

    /// Initial application setup when the event queue just started.
    fn setup(&self) {
        {
            // data must not outlive this scope. set_visible will trigger the update_size event which also borrow data.
            let mut data = self.data.borrow_mut();
            data.listen_events(self.notice.sender());

            self.mode_lbl.set_size(50, 30);
            self.window.set_text(&format!("SyncDraw - {}", data.instance_id));
            self.status.set_text(0, &format!("Current mode: {:?}; Instances linked: {}", data.mode, data.instances_count()));
        }

        self.window.set_visible(true);
    }
    
    /// Update the current mode of syndraw
    fn update_mode(&self, control: &nwg::Button) {
        let mut data = self.data.borrow_mut();

        if control == &self.pencil_btn {
            data.mode = AppMode::Draw;
        } else if control == &self.eraser_btn {
            data.mode = AppMode::Erase; 
        }

        self.status.set_text(0, &format!("Current mode: {:?}; Instances linked: {}", data.mode, data.instances_count()));
    }

    /// Update the drawing state of syncdraw
    fn update_draw(&self, evt: nwg::Event) {
        use nwg::Event as E;
        use nwg::MousePressEvent::*;

        let mut data = self.data.borrow_mut();

        match evt {
            E::OnMousePress(MousePressLeftDown) => { data.drawing = true; },
            E::OnMousePress(MousePressLeftUp) => { data.drawing = false; },
            _ => unreachable!()
        }
    }

    /// Resize the canvas to match the new window size. Also update the shared texture data.
    fn update_size(&self) {
        let (width, height) = self.canvas.size();
    
        // After the texture was resized write the new data to the shared memory
        self.canvas.resize_texture(width, height, true);
        self.update_texture();

        // Resize the viewport
        self.canvas.resize();
        self.canvas.render();

        // Saves the new window size and propagate the events to the other instances
        let data = self.data.borrow();
        data.set_window_size(self.window.size());
        data.sync();
    }

    /// Triggered when other instances of syncdraw updated the shared memory
    fn refresh(&self) {
        let data = self.data.borrow();
        let (w1, h1) = self.window.size();
        let (w2, h2) = data.window_size();

        // Update the window & canvas size
        if w1 != w2 || h1 != h2 {
            self.window.set_size(w2, h2);
            self.canvas.resize();
        }

        // Update the texture data
        let (w2, h2) = data.texture_size();
        let texture_data = data.texture_data();
        self.canvas.set_texture_data(w2, h2, &texture_data);
        self.canvas.render();
        self.status.set_text(0, &format!("Current mode: {:?}; Instances linked: {}", data.mode, data.instances_count()));
        self.invalidate_canvas();
    }

    /// Paint with the mouse in the canvas
    fn paint(&self) {
        let data = self.data.borrow();
        if !data.drawing { return; }

        let local_pos = nwg::GlobalCursor::local_position(&self.canvas, None);

        match data.mode {
            AppMode::Draw => self.canvas.paint(local_pos, [40, 40, 40, 255]),
            AppMode::Erase => self.canvas.paint(local_pos, [255, 255, 255, 255]),
        }

        self.canvas.render();
        self.update_texture();
        self.invalidate_canvas();
        data.sync();
    }

    /// Render the canvas content in the window.
    fn render(&self) {
        self.canvas.render();
    }

    /// Exit the application
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    /// Write the current texture in the shared memory
    fn update_texture(&self) {
        let (tw, th) = self.canvas.texture_size();
        let texture = self.canvas.texture_data();
        self.data.borrow().set_texture_data(tw, th, &texture);
    }

    /// Windows won't show the change to the canvas before the window is resized because it's Windows
    /// This fixes the issue.
    fn invalidate_canvas(&self) {
        use winapi::um::winuser::InvalidateRect;
        let handle = self.window.handle.hwnd().unwrap();
        unsafe { InvalidateRect(handle, ::std::ptr::null(), 1); }
    }
}


const ERR: &'static str = "Failed to initialize the SyncDraw";

/// Initialization that must be done after the Ui was created
/// This is only done if the instance of syncdraw is the first one created
fn init_shared_memory(app: &SyncDraw) {
    let data = app.data.borrow();
    if !data.first_instance() {
        return;
    }

    data.set_window_size(app.window.size());
    
    let (tw, th) = app.canvas.texture_size();
    let texture = app.canvas.texture_data();
    data.set_texture_data(tw, th, &texture);
}

/// Load the shared data into the instance
/// This is only done if the instance of syncdraw is not the first one created
fn load_shared_memory(app: &SyncDraw) {
    let data = app.data.borrow();
    if data.first_instance() {
        return;
    }

    let texture_data = data.texture_data();
    let (w, h) = data.texture_size();
    app.canvas.set_texture_data(w, h, &texture_data);
    app.canvas.render();

    let (w, h) = data.window_size();
    app.window.set_size(w, h);
    app.canvas.resize();
}

fn init_app() -> Result<(), &'static str> {
    nwg::init().map_err(|_e| ERR)?;

    let data = AppData::new();
    let app = SyncDraw { data: RefCell::new(data), ..Default::default() };
    let app = SyncDraw::build_ui(app).map_err(|_e| ERR)?;
    
    app.canvas.create_context().map_err(|_e| ERR)?;

    init_shared_memory(&app);
    load_shared_memory(&app);

    nwg::dispatch_thread_events();

    // Close the shared memory handle once we are done
    app.data.borrow_mut().close();

    Ok(())
}

fn main() {
    if let Err(e) = init_app() {
        nwg::error_message("Fatal error", e);
    }
}
