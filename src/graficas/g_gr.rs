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
  //? Calculamos los minimo sy maximos para un ajuste automatico
  if let Some((min, max)) = min_max_f64(&conf.r0) {
    x_min = min - 0.01;
    x_max = max + 0.01;
  }
  if let Some((min, max)) = min_max_f64(&conf.gr) {
    y_min = min - 0.01;
    y_max = max + 0.01;
  }
  //? Inicializamos la grafica, dando el directorio y el tamaño (ancho, alto)
  let root = BitMapBackend::new(
    &dir_grafica_gr, (720, 720)
  ).into_drawing_area();
  //? Dibujamos el fondo blanco
  let _ = root.fill(&WHITE);
  //? Añadimos un margen
  let root = root.margin(10, 10, 10, 10);

  let mut chart = ChartBuilder::on(&root)
    //? Titulo de la grafica
    .caption("GR", ("sans-serif", 24).into_font())
    //? Tamaño de las etiquetas de la grafica
    .x_label_area_size(20)
    .y_label_area_size(20)
    //? Construimos los limtes de la grafica
    .build_cartesian_2d(
      x_min..x_max, 
      y_min..y_max
    )?;

  chart
    //? Configuramos los datos de la grafica
    .configure_mesh()
    //? Configuramos los ejes
    .x_labels(5)
    .y_labels(5)
    //? Desactivamos la cuadricula
    .disable_mesh()
    //? Formateamos los ejes
    .y_label_formatter(&|x| format!("{:.2}", x))
    //? Dibujamos la grafica
    .draw()?;

  //? Dibujamos la línea de la gráfica
  chart.draw_series(
  LineSeries::new(
    conf.r0.iter().zip(conf.gr.iter()).map(|(&a, &b)| (a, b)).collect::<Vec<_>>(),
    Into::<ShapeStyle>::into(&PURPLE).stroke_width(2),
  ))?;

  //? Guardamos los cambios en la grafica
  root.present()?;
  //? Retornamos Ok si no hay errores
  Ok(())
}