/**
    Simple example on how to use a nwg canvas.
*/

#[macro_use] extern crate native_windows_gui as nwg;

use nwg::{Error, EventArgs, Ui, fatal_message, dispatch_events};
use nwg::constants::canvas;
use nwg::events as nwge;

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
        (Canvas, Paint, nwge::Paint, |app, _, _, _| {
            let mut canvas = nwg_get_mut!(app; (Canvas, nwg::Canvas<CanvasId>));

            // Get the renderer and clear the last scene
            let mut renderer = canvas.renderer(app).unwrap();
            renderer.clear(0.3, 0.3, 0.6, 1.0);
            
            // Drawing setup
            let (w, h) = renderer.get_render_size();
            renderer.set_transform( &[[1.0, 0.0], [0.0, 1.0], [w/2.0, h/2.0]] );

            let r1 = canvas::Rectangle{left: -50.0, right: 50.0, top: -50.0, bottom: 50.0};
            let r2 = canvas::Rectangle{left: -100.0, right: 100.0, top: -100.0, bottom: 100.0};
            let r3 = canvas::Rectangle{left: -200.0, right: 200.0, top: -200.0, bottom: 200.0};
            let e1 = canvas::Ellipse{ center: (0.0, 0.0), radius: (25.0, 25.0) };
            let e2 = canvas::Ellipse{ center: (0.0, 0.0), radius: (190.0, 190.0) };

            // Draw the shapes
            renderer.draw_ellipse(&SolidBrush(1), Some(&DashedPen(0)), &e2, 6.0).unwrap();
            renderer.draw_rectangle(&SolidBrush(2), None, &r3, 3.0).unwrap();
            renderer.fill_rectangle(&SolidBrush(1), &r2).unwrap();
            renderer.fill_rounded_rectangle(&SolidBrush(0), &r1, (15.0, 15.0)).unwrap();
            renderer.fill_ellipse(&SolidBrush(2), &e1).unwrap();
        }),

        (MainWindow, ResizeCanvas, nwge::Resized, |app, _, _, args| {
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

fn setup_canvas_resources(app: &Ui<CanvasId>) -> Result<(), Error> {
    use nwg::constants::canvas::*;

    let b1 = nwg::BrushT{
        canvas: Canvas, 
        btype: BrushType::SolidBrush(SolidBrush{color:(0.0, 0.7, 1.0, 1.0)})
    };
    let b2 = nwg::BrushT{
        canvas: Canvas,
        btype: BrushType::SolidBrush(SolidBrush{color:(0.0, 1.0, 0.5, 1.0)})
    };
    let b3 = nwg::BrushT{
        canvas: Canvas,
        btype: BrushType::SolidBrush(SolidBrush{color:(1.0, 1.0, 0.0, 1.0)}) 
    };
    let p1 = nwg::PenT{
        canvas: Canvas,
        start_cap: CapStyle::Round,
        end_cap: CapStyle::Round,
        dash_cap: CapStyle::Round,
        line_join: LineJoin::Round,
        miter_limit: 0.0,
        dash_style: DashStyle::Dash,
        dash_offset: 5.0
    };

    app.pack_resource(&SolidBrush(0), b1);
    app.pack_resource(&SolidBrush(1), b2);
    app.pack_resource(&SolidBrush(2), b3);
    app.pack_resource(&DashedPen(0), p1);

    app.commit()
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

    if let Err(e) = setup_canvas_resources(&app) {
        fatal_message("Fatal Error", &format!("{:?}", e));
    }

    dispatch_events();
}