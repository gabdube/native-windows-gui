use crate::*;
use std::cell::RefCell;


#[derive(Default)]
struct CanvasResources {
    fonts: WriteFactory,
    arial: WriteTextFormat,

    plain_stroke: StrokeStyle,
    background_brush: SolidBrush,

    header_border_brush: SolidBrush,
    header_background_brush: SolidBrush,
    header_title_brush: SolidBrush,

    plain_stroke2: StrokeStyle,
    header_close_brush: SolidBrush,
    header_close_border_brush: SolidBrush,

    hover_close: bool
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
    header: Canvas,
    close_button: Canvas
}

fn init_resources(canvas: &CanvasTest) {
    let mut res = canvas.resources.borrow_mut();

    const BASE_GRAY: Color = Color::rgb([0.25, 0.25, 0.25]);
    const DARK_GRAY: Color = Color::rgb([0.12, 0.12, 0.12]);
    const DARK_GRAY2: Color = Color::rgb([0.18, 0.18, 0.18]);
    const WHITEISH: Color = Color::rgb([0.8, 0.8, 0.8]);
    const RED1: Color = Color::rgb([0.7, 0.13, 0.13]);
    const RED2: Color = Color::rgb([0.5, 0.0, 0.0]);

    res.fonts = WriteFactory::new().expect("Failed to create write factory");
    res.arial = WriteTextFormat::builder(&res.fonts)
        .family("Arial")
        .size(14.0)
        .build()
        .expect("Failed to build text format");

    let header = &canvas.header;
    res.plain_stroke = StrokeStyle::from_style(header, DashStyle::Solid);
    res.header_border_brush = SolidBrush::from_color(header, DARK_GRAY);
    res.header_background_brush = SolidBrush::from_color(header, DARK_GRAY2);
    res.header_title_brush = SolidBrush::from_color(header, WHITEISH);

    let close = &canvas.close_button;
    res.plain_stroke2 = StrokeStyle::from_style(close, DashStyle::Solid);
    res.header_close_brush = SolidBrush::from_color(close, RED1);
    res.header_close_border_brush = SolidBrush::from_color(close, RED2);

    let window = &canvas.window;
    res.background_brush = SolidBrush::from_color(window, BASE_GRAY);
}

fn paint_window(canvas: &CanvasTest) {
    let res = canvas.resources.borrow();
    let can = &canvas.window;

    let draw = can.begin_draw();
    let (w, h) = draw.size();
    
    let background = Rect { left: 0.0, top: 0.0, right: w as f32, bottom: h as f32 };
    draw.fill_rectangle(&background, &res.background_brush);

    draw.end_draw().unwrap();
}

fn paint_header(canvas: &CanvasTest) {
    let res = canvas.resources.borrow();
    let can = &canvas.header;

    let draw = can.begin_draw();
    let (w, h) = draw.size();

    let header = Rect { left: 0.0, top: 0.0, right: w as f32, bottom: h as f32 };
    draw.fill_rectangle(&header, &res.header_background_brush);
    draw.draw_rectangle(&header, &res.header_border_brush, 3.0, &res.plain_stroke);
    draw.draw_simple_text("Canvas Test", &res.arial, (5.0, 8.0), (w, h), &res.header_title_brush);

    draw.end_draw().unwrap();
}

fn paint_close(canvas: &CanvasTest) {
    let res = canvas.resources.borrow();
    let can = &canvas.close_button;

    let draw = can.begin_draw();

    draw.clear(Color::rgb([0.18, 0.18, 0.18]));

    let close_button = Ellipse { point: Point2F { x: 10.0, y: 10.0 }, radiusX: 9.0, radiusY: 9.0};
    draw.fill_ellipse(&close_button, &res.header_close_brush);
    draw.draw_ellipse(&close_button, &res.header_close_border_brush, 1.0, &res.plain_stroke2);

    let e = draw.end_draw();
    match e {
        Err(CanvasError::Other(v)) => println!("{:X}", v),
        _ => {}
    }
}

fn drag_header(canvas: &CanvasTest) {
    let header = &canvas.header.handle;
    if GlobalCursor::dragging(header, None) {
        GlobalCursor::set_capture(header);

        let mut stuff = canvas.other_stuff.borrow_mut();
        stuff.anchor = GlobalCursor::local_position(header, None);
        stuff.dragging = true;
    }
}

fn drag(canvas: &CanvasTest) {
    let stuff = canvas.other_stuff.borrow();
    if stuff.dragging {
        let (x1, y1) = GlobalCursor::position();
        let (ax, ay) = stuff.anchor;
        canvas.window.set_position(x1 - ax, y1 - ay);
    }

    /*let mut res = canvas.resources.borrow_mut();
    let hover_close = {
        let (w, h) = canvas.header.size();
        let (x, y) = Cursor::local_position(&canvas.header, None);
        x > ((w as i32)-27)
    };
    if hover_close && !res.hover_close {
        res.header_close_brush.set_color(Color::rgb([0.95, 0.0, 0.0]));
        res.header_close_border_brush.set_color(Color::rgb([0.7, 0.0, 0.0]));
        res.hover_close = true;
    } else if !hover_close && res.hover_close {
        res.header_close_brush.set_color(Color::rgb([0.70, 0.13, 0.13]));
        res.header_close_border_brush.set_color(Color::rgb([0.5, 0.0, 0.0]));
        res.hover_close = false;
    }*/
}

fn release_header(canvas: &CanvasTest) {
    GlobalCursor::release();
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

            Canvas::builder()
                .size((20, 20))
                .position((275, 5))
                .parent(Some(&data.header))
                .build(&mut data.close_button)?;

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
                    else if &handle == &self.close_button { paint_close(self); }
                },
                E::MousePress(MousePressEvent::MousePressLeftDown) => {
                    if &handle == &self.header { drag_header(self); }
                },
                E::MousePress(MousePressEvent::MousePressLeftUp) => {
                    if &handle == &self.header { release_header(self); }
                },
                E::OnMouseMove => {
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
