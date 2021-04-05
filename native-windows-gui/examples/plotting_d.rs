/*!
    A example on how to use the `PlottingCanvas` for rendering figures, plots, and charts in native-windows-gui.
    To run: `cargo run --example plotting_d --features "plotting"`
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use nwg::stretch::{style::{*, Dimension::*}, geometry::*};
use std::time::Duration;
use plotters::prelude::*;


const EXAMPLES: &[&str] = &[
    "Simple",
    "Histogram",
    "Mandelbrot",
    "Multiple plot",
    "Animated",
];

#[derive(Default, NwgUi)]
pub struct PlottingExample {
    #[nwg_control(size: (900, 600), position: (300, 300), title: "Plotting")]
    #[nwg_events( OnInit: [PlottingExample::draw_graph], OnWindowClose: [nwg::stop_thread_dispatch()] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, min_size: [500, 200])]
    layout: nwg::GridLayout,

    #[nwg_control(parent: window)]
    #[nwg_events( OnResize: [PlottingExample::draw_graph] )]
    #[nwg_layout_item(layout: layout, col: 0, row: 0, col_span: 3)]
    graph: nwg::Plotters,

    #[nwg_control(parent: window, interval: Duration::from_millis(1000/60))]
    timer: nwg::AnimationTimer,

    #[nwg_control(parent: window)]
    #[nwg_layout_item(layout: layout, col: 0, col: 3)]
    options_frame: nwg::Frame,

    #[nwg_layout(
        parent: options_frame,
        auto_size: false,
        flex_direction: FlexDirection::Column,
        padding: Rect { start: Points(5.0), end: Points(5.0), top: Points(5.0), bottom: Points(5.0) }
    )]
    options_layout: nwg::FlexboxLayout,

    #[nwg_control(parent: options_frame, text: "Examples:")]
    #[nwg_layout_item(layout: options_layout, size: Size { width: Auto, height: Points(30.0) })]
    label1: nwg::Label,

    #[nwg_control(parent: options_frame, selected_index: Some(0), collection: EXAMPLES.to_vec())]
    #[nwg_events(OnListBoxSelect: [PlottingExample::draw_graph])]
    #[nwg_layout_item(layout: options_layout, size: Size { width: Auto, height: Points(200.0) })]
    example_list: nwg::ListBox<&'static str>,
}

impl PlottingExample {

    fn simple_chart(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = self.graph.draw().unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("y=x^2", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

        chart.configure_mesh().draw()?;

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
        let index = self.example_list.selection().unwrap_or(0);
        let result = match index {
            0 => self.simple_chart(),
            1 => self.simple_chart(),
            2 => self.simple_chart(),
            3 => self.simple_chart(),
            4 => self.simple_chart(),
            _ => unreachable!(),
        };

        if let Err(e) = result {
            let msg = format!("Error drawing chart: {:?}", e);
            nwg::modal_error_message(&self.window, "Error", &msg);
        }
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = PlottingExample::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
