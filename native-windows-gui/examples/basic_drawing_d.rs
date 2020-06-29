/*!
    Small custom control example using GDI. NWG has no drawing API, so raw winapi must be used.

    Requires the following features: `cargo run --example basic_drawing_d --features "extern-canvas"`
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use std::mem;
use std::cell::{RefCell, Cell};
use winapi::shared::windef::{HBRUSH, HPEN};
use winapi::um::wingdi::{CreateSolidBrush, CreatePen, Ellipse, Polygon, SelectObject, RGB, PS_SOLID};


pub struct PaintData {
    background: HBRUSH,
    border: HBRUSH,
    pen: HPEN,
    yellow: HBRUSH,
    white: HBRUSH,
    black: HBRUSH,
    red: HBRUSH,
}

impl Default for PaintData {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[derive(Default, NwgUi)]
pub struct DrawingApp {
    #[nwg_control(size: (300, 300), position: (300, 300), title: "Drawing (that's a duck)", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()], OnInit: [DrawingApp::setup])]
    window: nwg::Window,

    // By default ExternCanvas is a window so we must specify the parent here
    #[nwg_control(parent: Some(&data.window), position: (10, 10), size: (280, 280))]
    #[nwg_events( 
        OnPaint: [DrawingApp::paint(SELF, EVT_DATA)], 
        OnMousePress: [DrawingApp::events(SELF, EVT)],
    )]
    canvas: nwg::ExternCanvas,

    paint_data: RefCell<PaintData>,
    clicked: Cell<bool>,
}

impl DrawingApp {

    fn setup(&self) {
        let mut data = self.paint_data.borrow_mut();
        unsafe {
            data.background = CreateSolidBrush(RGB(190, 190, 255));
            data.border = CreateSolidBrush(RGB(100, 100, 255));
            data.pen = CreatePen(PS_SOLID as _, 2, RGB(20, 20, 20));
            data.yellow = CreateSolidBrush(RGB(255, 255, 0));
            data.white = CreateSolidBrush(RGB(255, 255, 255));
            data.black = CreateSolidBrush(RGB(10, 10, 10));
            data.red = CreateSolidBrush(RGB(255, 10, 0));
        }
    }

    fn events(&self, evt: nwg::Event) {
        use nwg::Event as E;
        use nwg::MousePressEvent as M;

        match evt {
            E::OnMousePress(M::MousePressLeftUp) => { self.clicked.set(false); },
            E::OnMousePress(M::MousePressLeftDown) => { self.clicked.set(true); },
            _ => {},
        }

        self.canvas.invalidate();
    }

    fn paint(&self, data: &nwg::EventData) {
        use winapi::um::winuser::{FillRect, FrameRect};
        use winapi::shared::windef::POINT as P;
        
        let paint = data.on_paint();
        let ps = paint.begin_paint();
        
        unsafe {
            let p = self.paint_data.borrow();
            let hdc = ps.hdc;
            let rc = &ps.rcPaint;

            FillRect(hdc, rc, p.background as _);
            FrameRect(hdc, rc, p.border as _);

            SelectObject(hdc, p.pen as _);
            SelectObject(hdc, p.yellow as _);
            Ellipse(hdc, rc.left + 20, rc.top + 20, rc.right - 20, rc.bottom - 20);

            SelectObject(hdc, p.white as _);
            Ellipse(hdc, 60, 60, 130, 130);
            Ellipse(hdc, 150, 60, 220, 130);

            if self.clicked.get() {
                SelectObject(hdc, p.red as _);
            } else {
                SelectObject(hdc, p.black as _);
            }
            
            Ellipse(hdc, 80, 80, 110, 110);
            Ellipse(hdc, 170, 80, 200, 110);

            SelectObject(hdc, p.red as _);
            let pts = &[P{x: 60, y: 150}, P{x: 220, y: 150}, P{x: 140, y: 220}];
            Polygon(hdc, pts.as_ptr(), pts.len() as _);
        }

        paint.end_paint(&ps);
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = DrawingApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}

