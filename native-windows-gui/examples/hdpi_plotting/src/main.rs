/*!
    A example on how to use the `PlottingCanvas` for rendering figures, plots, and charts in native-windows-gui.
    To run: `cargo run --example plotting_d --features "plotting"`
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
#[allow(unused, deprecated)]
use nwg::{NativeUi, set_dpi_awareness};
use std::{time::{Duration}};
use plotters::{prelude::*};

#[derive(Default, NwgUi)]
pub struct HdpiPlottingExample {
    #[nwg_control(size: (600, 600), position: (300, 300), title: "HdpiPlotting")]
    #[nwg_events(
        OnInit: [HdpiPlottingExample::draw_graph],
        OnWindowClose: [nwg::stop_thread_dispatch()],
        OnResize: [HdpiPlottingExample::draw_graph],
    )]
    window: nwg::Window,

    #[nwg_control(parent: window)]
    #[nwg_events(
        OnMouseMove: [HdpiPlottingExample::update_interactive],
    )]
    graph: nwg::Plotters,

    #[nwg_control(parent: window, interval: Duration::from_millis(1000/30))]
    #[nwg_events( OnTimerTick: [HdpiPlottingExample::draw_graph] )]
    animation_timer: nwg::AnimationTimer,
}

impl HdpiPlottingExample {

    fn simple_chart(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = self.graph.draw().unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("y=x^2", ("sans-serif", 50).into_font())
            .margin(15)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

        chart.configure_mesh()
            .light_line_style(ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 0 })
            .draw()?;

        chart
            .draw_series(LineSeries::new(
                (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                &RED,
            ))?
            .label("y = x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        Ok(())
    }

    fn draw_graph(&self) {
        if let Err(e) = self.simple_chart() {
            let msg = format!("Error drawing chart: {:?}", e);
            nwg::modal_error_message(&self.window, "Error", &msg);
        }
    }

    fn update_interactive(&self) {

    }
}

fn main() {
    // unsafe { set_dpi_awareness() };

    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = HdpiPlottingExample::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
