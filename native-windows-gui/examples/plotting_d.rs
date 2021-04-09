/*!
    A example on how to use the `PlottingCanvas` for rendering figures, plots, and charts in native-windows-gui.
    To run: `cargo run --example plotting_d --features "plotting"`
*/

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use nwg::stretch::{style::{*, Dimension::*}, geometry::*};
use std::{cell::RefCell, time::{Duration, Instant}};
use plotters::{prelude::*};


const EXAMPLES: &[&str] = &[
    "Simple",
    "Histogram",
    "Mandelbrot",
    "Multiple plot",
    "Animated",
    "Interactive",
    "Koch Snowflake",
];

#[derive(Default)]
pub struct PlottingData {
    animation_start: Option<Instant>,
}

#[derive(Default, NwgUi)]
pub struct PlottingExample {
    plotting_data: RefCell<PlottingData>,

    #[nwg_control(size: (900, 600), position: (300, 300), title: "Plotting")]
    #[nwg_events( 
        OnInit: [PlottingExample::draw_graph],
        OnWindowClose: [nwg::stop_thread_dispatch()],
        OnResize: [PlottingExample::draw_graph],
    )]
    window: nwg::Window,

    #[nwg_layout(parent: window, min_size: [400, 250])]
    layout: nwg::GridLayout,

    #[nwg_control(parent: window)]
    #[nwg_events( 
        OnMouseMove: [PlottingExample::update_interactive],
    )]
    #[nwg_layout_item(layout: layout, col: 0, row: 0, col_span: 3)]
    graph: nwg::Plotters,

    #[nwg_control(parent: window, interval: Duration::from_millis(1000/30))]
    #[nwg_events( OnTimerTick: [PlottingExample::draw_graph] )]
    animation_timer: nwg::AnimationTimer,

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
    #[nwg_events(OnListBoxSelect: [PlottingExample::check_animate, PlottingExample::draw_graph])]
    #[nwg_layout_item(layout: options_layout, size: Size { width: Auto, height: Points(200.0) })]
    example_list: nwg::ListBox<&'static str>,
}

impl PlottingExample {

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

    fn histogram_chart(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = self.graph.draw().unwrap();

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(25)
            .caption("Histogram Test", ("sans-serif", 50.0).into_font())
            .build_cartesian_2d((0u32..10u32).into_segmented(), 0u32..10u32)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .y_desc("Count")
            .x_desc("Bucket")
            .axis_desc_style(("sans-serif", 15))
            .draw()?;

        let data = [
            0u32, 1, 1, 1, 4, 2, 5, 7, 8, 6, 4, 2, 1, 8, 3, 3, 3, 4, 4, 3, 3, 3,
        ];

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(RED.mix(0.5).filled())
                .data(data.iter().map(|x: &u32| (*x, 1))),
        )?;

        Ok(())
    }

    fn mandelbrot_chart(&self) -> Result<(), Box<dyn std::error::Error>> {
        use std::ops::Range;

        let root = self.graph.draw().unwrap();

        fn mandelbrot_set(
            real: Range<f64>,
            complex: Range<f64>,
            samples: (usize, usize),
            max_iter: usize,
        ) -> impl Iterator<Item = (f64, f64, usize)> {
            let step = (
                (real.end - real.start) / samples.0 as f64,
                (complex.end - complex.start) / samples.1 as f64,
            );
            return (0..(samples.0 * samples.1)).map(move |k| {
                let c = (
                    real.start + step.0 * (k % samples.0) as f64,
                    complex.start + step.1 * (k / samples.0) as f64,
                );
                let mut z = (0.0, 0.0);
                let mut cnt = 0;
                while cnt < max_iter && z.0 * z.0 + z.1 * z.1 <= 1e10 {
                    z = (z.0 * z.0 - z.1 * z.1 + c.0, 2.0 * z.0 * z.1 + c.1);
                    cnt += 1;
                }
                return (c.0, c.1, cnt);
            });
        }

        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .x_label_area_size(10)
            .y_label_area_size(10)
            .build_cartesian_2d(-2.1f64..0.6f64, -1.2f64..1.2f64)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .draw()?;

        let plotting_area = chart.plotting_area();

        let range = plotting_area.get_pixel_range();

        let (pw, ph) = (range.0.end - range.0.start, range.1.end - range.1.start);
        let (xr, yr) = (chart.x_range(), chart.y_range());

        for (x, y, c) in mandelbrot_set(xr, yr, (pw as usize, ph as usize), 100) {
            if c != 100 {
                plotting_area.draw_pixel((x, y), &HSLColor(c as f64 / 100.0, 1.0, 0.5))?;
            } else {
                plotting_area.draw_pixel((x, y), &BLACK)?;
            }
        }

        Ok(())
    }

    fn multiple_plot(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = self.graph.draw().unwrap();


        let root_area = root.titled("Multiplot", ("sans-serif", 60))?;
        let (upper, lower) = root_area.split_vertically(256);

        let x_axis = (-3.4f32..3.4).step(0.1);
        let mut cc = ChartBuilder::on(&upper)
            .margin(5)
            .set_all_label_area_size(50)
            .caption("Sine and Cosine", ("sans-serif", 40))
            .build_cartesian_2d(-3.4f32..3.4, -1.2f32..1.2f32)?;

        cc.configure_mesh()
            .x_labels(20)
            .y_labels(10)
            .disable_mesh()
            .x_label_formatter(&|v| format!("{:.1}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .draw()?;

        cc.draw_series(LineSeries::new(x_axis.values().map(|x| (x, x.sin())), &RED))?
            .label("Sine")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        cc.draw_series(LineSeries::new(
            x_axis.values().map(|x| (x, x.cos())),
            &BLUE,
        ))?
        .label("Cosine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        cc.configure_series_labels().border_style(&BLACK).draw()?;


        let drawing_areas = lower.split_evenly((1, 2));

        for (drawing_area, idx) in drawing_areas.iter().zip(1..) {
            let mut cc = ChartBuilder::on(&drawing_area)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .margin_right(20)
                .caption(format!("y = x^{}", 1 + 2 * idx), ("sans-serif", 40))
                .build_cartesian_2d(-1f32..1f32, -1f32..1f32)?;
            cc.configure_mesh().x_labels(5).y_labels(3).draw()?;

            cc.draw_series(LineSeries::new(
                (-1f32..1f32)
                    .step(0.01)
                    .values()
                    .map(|x| (x, x.powf(idx as f32 * 2.0 + 1.0))),
                &BLUE,
            ))?;
        }

        Ok(())
    }

    fn animated_chart(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = self.graph.draw().unwrap();

        let time = self.plotting_data.borrow_mut()
            .animation_start
            .as_ref()
            .map(|t| t.elapsed())
            .unwrap_or(Duration::new(0, 0));

        let offset = (time.as_millis() as f64) / 1000.0;
        let x_spec = (0.0+offset)..(5.0+offset);
        
        let sin_x_values = (0..=250)
            .map(|x| x as f64 / 50.0)
            .map(|x| ((x+offset), (x+offset).sin()));

        let cos_x_values = (0..=250)
            .map(|x| x as f64 / 50.0)
            .map(|x| ((x+offset), (x+offset).cos()));

        let mut chart = ChartBuilder::on(&root)
            .caption("SIN & COS", ("sans-serif", 50).into_font())
            .margin(15)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_spec, -1.5f64..1.5f64)?;

        chart.configure_mesh()
            .x_labels(20)
            .y_labels(10)
            .disable_mesh()
            .draw()?;

        chart
            .draw_series(LineSeries::new(sin_x_values, &RED))?
            .label("y = sin(x)")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .draw_series(LineSeries::new(cos_x_values, &BLUE))?
            .label("y = cos(x)")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;


        Ok(())
    }

    fn interactive_chart(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = self.graph.draw().unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption("y=x*2", ("sans-serif", 50).into_font())
            .margin(50)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-100..100, -200..200)?;


        chart.configure_mesh()
            .light_line_style(ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 0 })
            .draw()?;

        
        chart
            .draw_series(LineSeries::new(
                [-100, -50, 0, 50, 100].iter().map(|&x| (x, x * 2)),
                &RED,
            ))?
            .label("y = x*2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart.configure_series_labels()
            .border_style(&BLACK)
            .draw()?;
        
        
        // As far as I know, there's no way to fetch the margin in pixels within a chart, so you have to use trial and error
        // 80 & 130 seems to be good enough for this case
        let (x, _) = nwg::GlobalCursor::local_position(&self.graph, None);
        let max_x = self.graph.size().0 as i32;
        let percent = (x-80) as f32 / (max_x-130) as f32;
        let value = (((percent - 0.5) * 200.0) as i32).clamp(-100, 100);

        chart.draw_series(PointSeries::of_element(
            [value].iter().map(|&x| (x, x * 2)),
            5,
            ShapeStyle::from(&RED).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style)
                    + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
            },
        ))?;

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        Ok(())
    }

    fn kotch_snowflake(&self) ->  Result<(), Box<dyn std::error::Error>> {
        let root = self.graph.draw().unwrap();

        fn snowflake_iter(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
            let mut ret = vec![];
            for i in 0..points.len() {
                let (start, end) = (points[i], points[(i + 1) % points.len()]);
                let t = ((end.0 - start.0) / 3.0, (end.1 - start.1) / 3.0);
                let s = (
                    t.0 * 0.5 - t.1 * (0.75f64).sqrt(),
                    t.1 * 0.5 + (0.75f64).sqrt() * t.0,
                );
                ret.push(start);
                ret.push((start.0 + t.0, start.1 + t.1));
                ret.push((start.0 + t.0 + s.0, start.1 + t.1 + s.1));
                ret.push((start.0 + t.0 * 2.0, start.1 + t.1 * 2.0));
            }
            ret
        }

        let mut chart = ChartBuilder::on(&root)
            .caption("Koch's Snowflake", ("sans-serif", 50))
            .build_cartesian_2d(-2.0..2.0, -1.5..1.5)?;

        let mut snowflake_vertices = {
            let mut current: Vec<(f64, f64)> = vec![
                (0.0, 1.0),
                ((3.0f64).sqrt() / 2.0, -0.5),
                (-(3.0f64).sqrt() / 2.0, -0.5),
            ];
            for _ in 0..6 {
                current = snowflake_iter(&current[..]);
            }
            current
        };

        chart.draw_series(std::iter::once(Polygon::new(
            snowflake_vertices.clone(),
            &RED.mix(0.2),
        )))?;
        snowflake_vertices.push(snowflake_vertices[0]);
        chart.draw_series(std::iter::once(PathElement::new(snowflake_vertices, &RED)))?;

        Ok(())
    }

    fn draw_graph(&self) {
        let index = self.example_list.selection().unwrap_or(0);

        let result = match index {
            0 => self.simple_chart(),
            1 => self.histogram_chart(),
            2 => self.mandelbrot_chart(),
            3 => self.multiple_plot(),
            4 => self.animated_chart(),
            5 => self.interactive_chart(),
            6 => self.kotch_snowflake(),
            _ => unreachable!(),
        };

        if let Err(e) = result {
            let msg = format!("Error drawing chart: {:?}", e);
            nwg::modal_error_message(&self.window, "Error", &msg);
        }
    }

    fn check_animate(&self) {
        let index = self.example_list.selection().unwrap_or(0);
        if index == 4 {
            self.plotting_data.borrow_mut().animation_start = Some(Instant::now());
            self.animation_timer.start();
        } else {
            self.animation_timer.stop();
        }
    }

    fn update_interactive(&self) {
        let index = self.example_list.selection().unwrap_or(0);
        if index == 5 {
            let result = self.interactive_chart();
            if let Err(e) = result {
                let msg = format!("Error drawing chart: {:?}", e);
                nwg::modal_error_message(&self.window, "Error", &msg);
            }
        }
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = PlottingExample::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
