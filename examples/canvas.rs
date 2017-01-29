/**
    Simple example on how to use a nwg canvas.
*/

#[macro_use] extern crate native_windows_gui as nwg;

use nwg::{Event, EventArgs, Ui, fatal_message, dispatch_events};
use nwg::constants as nwgc;

nwg_template!(
    head: setup_ui<&'static str>,
    controls: [
        ("MainWindow", nwg_window!( title="Template Example"; size=(500, 500); resizable=true )),
        ("Canvas", nwg_canvas!( parent="MainWindow"; size=(500,500)))
    ];
    events: [
        ("Canvas", "Paint", Event::Paint, |app, _, _, _| {
            let mut canvas = nwg_get_mut!(app; ("Canvas", nwg::Canvas<&'static str>));

            // Get the renderer and clear the last scene
            let mut renderer = canvas.renderer().unwrap();
            renderer.clear(0.3, 0.3, 0.6, 1.0);
            
            // Drawing setup
            let (w, h) = renderer.get_render_size();
            renderer.set_transform( [[1.0, 0.0], [0.0, 1.0], [w/2.0, h/2.0]] );

            let r1 = nwgc::Rectangle{left: -50.0, right: 50.0, top: -50.0, bottom: 50.0};
            let r2 = nwgc::Rectangle{left: -100.0, right: 100.0, top: -100.0, bottom: 100.0};
            let r3 = nwgc::Rectangle{left: -200.0, right: 200.0, top: -200.0, bottom: 200.0};
            let e1 = nwgc::Ellipse{ center: (0.0, 0.0), radius: (25.0, 25.0) };

            // Draw the shapes
            renderer.draw_rectangle(&"SolidBrush3", &r3, 3.0, None).unwrap();
            renderer.fill_rectangle(&"SolidBrush2", &r2).unwrap();
            renderer.fill_rounded_rectangle(&"SolidBrush1", &r1, (15.0, 15.0)).unwrap();
            renderer.fill_ellipse(&"SolidBrush3", &e1).unwrap();
        }),

        ("MainWindow", "ResizeCanvas", Event::Resized, |app, _, _, args| {
            match args {
                &EventArgs::Size(w, h) => {
                    let mut canvas = nwg_get_mut!(app; ("Canvas", nwg::Canvas<&'static str>));
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

fn setup_canvas_resources(app: &Ui<&'static str>) {
    let mut canvas = nwg_get_mut!(app; ("Canvas", nwg::Canvas<&'static str>));

    let b1 = nwgc::SolidBrush{color:(0.0, 0.7, 1.0, 1.0)};
    let b2 = nwgc::SolidBrush{color:(0.0, 1.0, 0.5, 1.0)};
    let b3 = nwgc::SolidBrush{color:(1.0, 1.0, 0.0, 1.0)};

    canvas.create_solid_brush(&"SolidBrush1", &b1).expect("Failed to import brush 1");
    canvas.create_solid_brush(&"SolidBrush2", &b2).expect("Failed to import brush 2");
    canvas.create_solid_brush(&"SolidBrush3", &b3).expect("Failed to import brush 3");
}

fn main() {
    let app: Ui<&'static str>;

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
