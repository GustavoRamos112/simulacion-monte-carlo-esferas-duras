use plotters::{prelude::*, style::full_palette::PURPLE};

use super::max_min::min_max_f64;

use crate::configuracion::variables::Configuracion;

pub fn g_gr(conf: &Configuracion, dir_grafica_gr: &str) 
-> Result<(), Box<dyn std::error::Error>> 
{
  let mut y_min: f64 = 0.0;
  let mut y_max: f64 = 1.0;
  let mut x_min: f64 = 0.0;
  let mut x_max: f64 = 1.0;
  
  if let Some((min, max)) = min_max_f64(&conf.r0) {
    x_min = min - 0.01;
    x_max = max + 0.01;
  }

  if let Some((min, max)) = min_max_f64(&conf.gr) {
    y_min = min - 0.01;
    y_max = max + 0.01;
  }

  let root = BitMapBackend::new(
    &dir_grafica_gr, (720, 720)
  ).into_drawing_area();
  let _ = root.fill(&WHITE);
  let root = root.margin(10, 10, 10, 10);
  
  let mut chart = ChartBuilder::on(&root)
    .caption("GR", ("sans-serif", 24).into_font())
    .x_label_area_size(20)
    .y_label_area_size(20)
    .build_cartesian_2d(
      x_min..x_max, 
      y_min..y_max
    )?;

  chart
    .configure_mesh()
    .x_labels(5)
    .y_labels(5)
    .disable_mesh()
    .y_label_formatter(&|x| format!("{:.2}", x))
    .draw()?;

  chart.draw_series(
  LineSeries::new(
    conf.r0.iter().zip(conf.gr.iter()).map(|(&a, &b)| (a, b)).collect::<Vec<_>>(),
    Into::<ShapeStyle>::into(&PURPLE).stroke_width(2),
  ))?;
  
  root.present()?;
  Ok(())
}