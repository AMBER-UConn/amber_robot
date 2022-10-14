use plotters::prelude::*;

use crate::bezier;
use crate::path_gen;

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
        .build_cartesian_2d(-500f32..500f32, -600f32..200f32)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    const num_steps: i32 = 100;
    
    let coefs = vec![(-170.0, -470.0), (-240.0, -470.0), (-300.0, -360.0), (-300.0, -360.0), 
                                            (-300.0, -360.0), (0.0, -360.0), (0.0, -360.0), (0.0, -320.0), 
                                            (300.0, -320.0), (300.0, -320.0), (240.0, -470.0), (170.0, -470.0), (170.0, -500.0), (0.0, -500.0), (-170.0, -470.0)];
    // let coefs = vec![(100.0, 500.0), (400.0, 2000.0), (1500.5, 1200.0), (1400.0, 900.0)];

    chart
        .draw_series(LineSeries::new(
            (0..num_steps+1).map(|t: i32| {
                (
                    // bezier::curve(t as f32 / num_steps as f32).0,
                    // bezier::curve(t as f32 / num_steps as f32).1,

                    // path_gen::decasteljau(t as f32 / num_steps as f32, &coefs).0,
                    // path_gen::decasteljau(t as f32 / num_steps as f32, &coefs).1,

                    // path_gen::new_eq(300.0, 100.0, num_steps as f32, t as f32).0,
                    // path_gen::new_eq(300.0, 100.0, num_steps as f32, t as f32).1
                    
                    // Tm = Tn + 2Te
                    path_gen::new_eq_2(300.0, 100.0, num_steps as f32, 40.0, 20.0, t as f32).0,
                    path_gen::new_eq_2(300.0, 100.0, num_steps as f32, 40.0, 20.0, t as f32).1
                )
            }),
            &RED,
        ))
        .unwrap();
}
