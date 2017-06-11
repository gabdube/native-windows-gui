/**
    Simple example on how to use a nwg canvas.
*/

#[macro_use] extern crate native_windows_gui as nwg;

use nwg::{Event, EventArgs, Ui, fatal_message, dispatch_events};
use nwg::constants as nwgc;

#[derive(Debug, Clone, Hash)]
pub enum CanvasId {
    // Controls
    MainWindow,
    Canvas,

    // Event
    Paint,
    ResizeCanvas,

    // Canvas resources
    SolidBrush(u8),
    DashedPen(u8)
}

use CanvasId::*;

nwg_template!(
    head: setup_ui<CanvasId>,
    controls: [
        (MainWindow, nwg_window!( title="Canvas Example"; size=(500, 500); resizable=true )),
        (Canvas, nwg_canvas!( parent=MainWindow; size=(500,500)))
    ];
    events: [
        (Canvas, Paint, Event::Paint, |app, _, _, _| {
            let mut canvas = nwg_get_mut!(app; (Canvas, nwg::Canvas<CanvasId>));

            // Get the renderer and clear the last scene
            let mut renderer = canvas.renderer().unwrap();
            renderer.clear(0.3, 0.3, 0.6, 1.0);
            
            // Drawing setup
            let (w, h) = renderer.get_render_size();
            renderer.set_transform( &[[1.0, 0.0], [0.0, 1.0], [w/2.0, h/2.0]] );

            let r1 = nwgc::Rectangle{left: -50.0, right: 50.0, top: -50.0, bottom: 50.0};
            let r2 = nwgc::Rectangle{left: -100.0, right: 100.0, top: -100.0, bottom: 100.0};
            let r3 = nwgc::Rectangle{left: -200.0, right: 200.0, top: -200.0, bottom: 200.0};
            let e1 = nwgc::Ellipse{ center: (0.0, 0.0), radius: (25.0, 25.0) };
            let e2 = nwgc::Ellipse{ center: (0.0, 0.0), radius: (190.0, 190.0) };

            // Draw the shapes
            renderer.draw_ellipse(&SolidBrush(1), Some(&DashedPen(0)), &e2, 6.0).unwrap();
            renderer.draw_rectangle(&SolidBrush(2), None, &r3, 3.0).unwrap();
            renderer.fill_rectangle(&SolidBrush(1), &r2).unwrap();
            renderer.fill_rounded_rectangle(&SolidBrush(0), &r1, (15.0, 15.0)).unwrap();
            renderer.fill_ellipse(&SolidBrush(2), &e1).unwrap();
        }),

        (MainWindow, ResizeCanvas, Event::Resized, |app, _, _, args| {
            match args {
                &EventArgs::Size(w, h) => {
                    let mut canvas = nwg_get_mut!(app; (Canvas, nwg::Canvas<CanvasId>));
                    canvas.set_size(w, h);
                    canvas.set_render_size(w, h);
                },
                _ => unreachable!()
            }
        })
    ];
    resources: [];
    values: []
);

fn setup_canvas_resources(app: &Ui<CanvasId>) {
    let mut canvas = nwg_get_mut!(app; (Canvas, nwg::Canvas<CanvasId>));

    let b1 = nwgc::SolidBrush{color:(0.0, 0.7, 1.0, 1.0)};
    let b2 = nwgc::SolidBrush{color:(0.0, 1.0, 0.5, 1.0)};
    let b3 = nwgc::SolidBrush{color:(1.0, 1.0, 0.0, 1.0)};
    let p1 = nwgc::Pen {
        start_cap: nwgc::CapStyle::Round,
        end_cap: nwgc::CapStyle::Round,
        dash_cap: nwgc::CapStyle::Round,
        line_join: nwgc::LineJoin::Round,
        miter_limit: 0.0,
        dash_style: nwgc::DashStyle::Dash,
        dash_offset: 5.0
    };

    canvas.create_solid_brush(&SolidBrush(0), &b1).expect("Failed to create brush 1");
    canvas.create_solid_brush(&SolidBrush(1), &b2).expect("Failed to create brush 2");
    canvas.create_solid_brush(&SolidBrush(2), &b3).expect("Failed to create brush 3");
    canvas.create_pen(&DashedPen(0), &p1).expect("Failed to create pen");
}

fn main() {
    let app: Ui<CanvasId>;

    match Ui::new() {
        Ok(_app) => { app = _app;  },
        Err(e) => { fatal_message("Fatal Error", &format!("{:?}", e) ); }
    }

    if let Err(e) = setup_ui(&app) {
        fatal_message("Fatal Error", &format!("{:?}", e));
    }

    setup_canvas_resources(&app);

    dispatch_events();
}
