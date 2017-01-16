/**
    Simple example on how to use a nwg canvas.
*/

#[macro_use] extern crate native_windows_gui as nwg;

use nwg::{Event, EventArgs, Ui, fatal_message, dispatch_events};

nwg_template!(
    head: setup_ui<&'static str>,
    controls: [
        ("MainWindow", nwg_window!( title="Template Example"; size=(500, 500); resizable=true )),
        ("Canvas", nwg_canvas!( parent="MainWindow"; size=(500,500)))
    ];
    events: [
        ("Canvas", "Paint", Event::Paint, |app, _, _, _| {
            let mut canvas = nwg_get_mut!(app; ("Canvas", nwg::Canvas));
            let mut renderer = canvas.renderer().unwrap();

            let render_size = renderer.render_size();

            renderer.clear(1.0, 1.0, 1.0, 1.0);
        }),

        ("MainWindow", "ResizeCanvas", Event::Resized, |app, _, _, args| {
            match args {
                &EventArgs::Size(w, h) => {
                    let canvas = nwg_get!(app; ("Canvas", nwg::Canvas));
                    canvas.set_size(w, h);
                },
                _ => unreachable!()
            }
        })
    ];
    resources: [
    ];
    values: [
    ]
);

fn main() {
    let app: Ui<&'static str>;

    match Ui::new() {
        Ok(_app) => { app = _app; },
        Err(e) => { fatal_message("Fatal Error", &format!("{:?}", e) ); }
    }

    if let Err(e) = setup_ui(&app) {
        fatal_message("Fatal Error", &format!("{:?}", e));
    }

    dispatch_events();
}
