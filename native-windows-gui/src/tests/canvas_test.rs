use crate::*;
use std::cell::RefCell;


#[derive(Default)]
struct CanvasResources {
    plain_stroke: StrokeStyle,
    background_brush: SolidBrush,
    header_border_brush: SolidBrush,
    header_gradient: GradientStopCollection
}


#[derive(Default)]
pub struct CanvasTest {
    resources: RefCell<CanvasResources>,
    pub window: CanvasWindow,
}

fn init_resources(canvas: &CanvasTest) {
    let mut res = canvas.resources.borrow_mut();
    let can = &canvas.window;

    const BASE_GRAY: Color = Color {r: 0.25, g: 0.25, b: 0.25, a: 1.0};
    const DARK_GRAY: Color = Color {r: 0.10, g: 0.10, b: 0.10, a: 1.0};

    res.plain_stroke = StrokeStyle::from_style(can, DashStyle::Solid);
    res.background_brush = SolidBrush::from_color(can, BASE_GRAY);
    res.header_border_brush = SolidBrush::from_color(can, DARK_GRAY);

    res.header_gradient = GradientStopCollection::from_stops(
        can,
        &[
            GradientStop {position: 0.0, color: BASE_GRAY},
            GradientStop {position: 0.7, color: DARK_GRAY},
            GradientStop {position: 1.0, color: BASE_GRAY}
        ]
    );

    println!("{:?}", res.header_gradient);

    /*res.header_inner_brush = LinearGradiantBrush::from_linear_gradient(
        can,
        LinearBrushProperties { startPoint: Point2F {x:150.0, y:30.0}, endPoint: Point2F {x:150.0, y:0.0} },
        &[  ]
    );*/
}

fn paint(canvas: &CanvasTest) {
    let res = canvas.resources.borrow();
    let can = &canvas.window;

    let draw = can.begin_draw();

    let (w, h) = draw.size();
    
    let background = Rect { left: 0.0, top: 0.0, right: w as f32, bottom: h as f32 };
    draw.fill_rectangle(&background, &res.background_brush);

    let header = Rect { left: 0.0, top: 0.0, right: w as f32, bottom: 30.0 };
    draw.draw_rectangle(&header, &res.header_border_brush, 1.0, &res.plain_stroke);

    if let Err(e) = draw.end_draw() {
        println!("{:?}", e);
    }
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

            Ok(())
        }

        fn process_event<'a>(&self, evt: Event, mut _evt_data: &EventData, handle: ControlHandle) {
            use crate::Event as E;

            match evt {
                E::OnInit => 
                    if &handle == &self.window {
                        init_resources(self);
                    },
                E::OnPaint => 
                    if &handle == &self.window {
                        paint(self);
                    },
                _ => {}
            }
        }

        fn handles(&self) -> Vec<&ControlHandle> {
            vec![&self.window.handle]
        }

    }

}
