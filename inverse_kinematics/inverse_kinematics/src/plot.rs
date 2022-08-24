use plotters::prelude::*;

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