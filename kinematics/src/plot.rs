use plotters::prelude::*;
// use crate::{path_gen::{self, walk_curve}, bezier};
use crate::bezier;


pub fn graph() {
  let drawing_area = BitMapBackend::new("test.png", (600, 400))
    .into_drawing_area();

  drawing_area.fill(&WHITE).unwrap();
  
  let mut chart = ChartBuilder::on(&drawing_area)
    .caption("Kinematics", ("Arial", 30))
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .build_cartesian_2d(0..100, 0..100)
    .unwrap();

    chart.configure_mesh().draw().unwrap();

  chart.draw_series(
    LineSeries::new((0..100).map(|x| (x, 100 - x)), &BLACK),
  ).unwrap();
  
}

pub fn curve_plot() {
  let drawing_area = BitMapBackend::new("test.png", (600, 400))
    .into_drawing_area();

  drawing_area.fill(&WHITE).unwrap();
  
  let mut chart = ChartBuilder::on(&drawing_area)
    .caption("Bezier Curve", ("Arial", 30))
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .build_cartesian_2d(0..40, 0..40)
    .unwrap();

    chart.configure_mesh().draw().unwrap();

  chart.draw_series(
    LineSeries::new((0..=10).map(|t:f32| (bezier::curve(t).0, bezier::curve(t).1)), &RED),
  ).unwrap();

  
}