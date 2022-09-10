use plotters::prelude::*;

use crate::bezier;
// use crate::{path_gen::{self, walk_curve}, bezier};

pub fn graph() {
    let drawing_area = BitMapBackend::new("test.png", (600, 400)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption("Kinematics", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .build_cartesian_2d(0..100, 0..100)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new((0..100).map(|x| (x, 100 - x)), &BLACK))
        .unwrap();
}

pub fn curve_plot() {
    let drawing_area = BitMapBackend::new("test.png", (600, 400)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption("Bezier Curve", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .build_cartesian_2d(-0f32..10f32, 0f32..10f32)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    const num_steps: i32 = 100;
    
    chart
        .draw_series(LineSeries::new(
            (0..num_steps).map(|t: i32| {
                (
                    bezier::curve(t as f32 / num_steps as f32).0,
                    bezier::curve(t as f32 / num_steps as f32).1,
                )
            }),
            &RED,
        ))
        .unwrap();
}
