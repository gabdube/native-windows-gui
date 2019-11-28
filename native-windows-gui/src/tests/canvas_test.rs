use crate::*;
use std::cell::RefCell;


#[derive(Default)]
struct CanvasResources {
    plain_stroke: StrokeStyle,
    background_brush: SolidBrush,

    header_border_brush: SolidBrush,
    header_background_brush: SolidBrush,
}

#[derive(Default)]
struct OtherStuff {
    dragging: bool,
    anchor: (i32, i32)
}


#[derive(Default)]
pub struct CanvasTest {
    resources: RefCell<CanvasResources>,
    other_stuff: RefCell<OtherStuff>,
    pub window: CanvasWindow,
    pub header: Canvas,
}

fn init_resources(canvas: &CanvasTest) {
    let mut res = canvas.resources.borrow_mut();
    let window = &canvas.window;
    let header = &canvas.header;

    const BASE_GRAY: Color = Color::rgb([0.25, 0.25, 0.25]);
    const DARK_GRAY: Color = Color::rgb([0.12, 0.12, 0.12]);
    const DARK_GRAY2: Color = Color::rgb([0.18, 0.18, 0.18]);

    res.plain_stroke = StrokeStyle::from_style(header, DashStyle::Solid);
    res.header_border_brush = SolidBrush::from_color(header, DARK_GRAY);
    res.header_background_brush = SolidBrush::from_color(header, DARK_GRAY2);

    res.background_brush = SolidBrush::from_color(window, BASE_GRAY);
}

fn paint_window(canvas: &CanvasTest) {
    let res = canvas.resources.borrow();
    let can = &canvas.window;

    let draw = can.begin_draw();

    let (w, h) = draw.size();
    
    let background = Rect { left: 0.0, top: 0.0, right: w as f32, bottom: h as f32 };
    draw.fill_rectangle(&background, &res.background_brush);

    if let Err(e) = draw.end_draw() {
        println!("{:?}", e);
    }
}

fn paint_header(canvas: &CanvasTest) {
    let res = canvas.resources.borrow();
    let can = &canvas.header;

    let draw = can.begin_draw();
    let (w, h) = draw.size();

    let header = Rect { left: 0.0, top: 0.0, right: w as f32, bottom: h as f32 };
    draw.fill_rectangle(&header, &res.header_background_brush);
    draw.draw_rectangle(&header, &res.header_border_brush, 3.0, &res.plain_stroke);

    if let Err(e) = draw.end_draw() {
        println!("{:?}", e);
    }
}

fn drag_header(canvas: &CanvasTest) {
    let header = &canvas.header.handle;
    if Cursor::dragging(header, None) {
        Cursor::set_capture(header);

        let mut stuff = canvas.other_stuff.borrow_mut();
        stuff.anchor = Cursor::local_position(header, None);
        stuff.dragging = true;
    }
}

fn drag(canvas: &CanvasTest) {
    let stuff = canvas.other_stuff.borrow();
    if stuff.dragging {
        let (x1, y1) = Cursor::position();
        let (ax, ay) = stuff.anchor;
        canvas.window.set_position(x1 - ax, y1 - ay);
    }
}

fn release_header(canvas: &CanvasTest) {
    Cursor::release();
    canvas.other_stuff.borrow_mut().dragging = false;
}


mod partial_canvas_test_ui {
    use super::*;
    use crate::{PartialUi, SystemError, ControlHandle};

    impl PartialUi<CanvasTest> for CanvasTest {

        fn build_partial<W: Into<ControlHandle>>(data: &mut CanvasTest, _parent: Option<W>) -> Result<(), SystemError> {
            CanvasWindow::builder()
                .flags(CanvasWindowFlags::POPUP)
                .size((300, 300))
                .position((250, 100))
                .title("Canvas")
                .build(&mut data.window)?;

            Canvas::builder()
                .size((300, 30))
                .position((0, 0))
                .parent(Some(&data.window))
                .build(&mut data.header)?;

            Ok(())
        }

        fn process_event<'a>(&self, evt: Event, mut _evt_data: &EventData, handle: ControlHandle) {
            use crate::Event as E;

            match evt {
                E::OnInit => 
                    if &handle == &self.window {
                        init_resources(self);
                    },
                E::OnPaint => {
                    if &handle == &self.window { paint_window(self); }
                    else if &handle == &self.header { paint_header(self); }
                },
                E::MousePress(MousePressEvent::MousePressLeftDown) => {
                    if &handle == &self.header { drag_header(self); }
                },
                E::MousePress(MousePressEvent::MousePressLeftUp) => {
                    if &handle == &self.header { release_header(self); }
                },
                E::MouseMove => {
                    if &handle == &self.header { drag(self); }
                },
                _ => {}
            }
        }

        fn handles(&self) -> Vec<&ControlHandle> {
            vec![&self.window.handle]
        }

    }

}
