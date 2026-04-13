use plotters::{prelude::*, style::full_palette::PURPLE};

use super::max_min::min_max_f64;

use crate::configuracion::variables::Configuracion;

pub fn g_presion(
  conf: &Configuracion,
  dir_grafica_presion: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let mut y_min: f64 = 0.0;
  let mut y_max: f64 = 1.0;
  let mut x_min: f64 = 0.0;
  let mut x_max: f64 = 1.0;
  //? Calculamos los minimo sy maximos para un ajuste automatico
  if let Some((min, max)) = min_max_f64(&conf.dp) {
    x_min = min - 0.01;
    x_max = max + 0.01;
  }
  if let Some((min, max)) = min_max_f64(&conf.pvnkt) {
    y_min = min - 0.01;
    y_max = max + 0.01;
  }
  //? Inicializamos la grafica, dando el directorio y el tamaño (ancho, alto)
  let root = BitMapBackend::new(&dir_grafica_presion, (720, 720)).into_drawing_area();
  //? Dibujamos el fondo blanco
  let _ = root.fill(&WHITE);
  //? Añadimos un margen
  let root = root.margin(10, 10, 10, 10);

  let mut chart = ChartBuilder::on(&root)
    //? Titulo de la grafica
    .caption("PVNKT", ("sans-serif", 24).into_font())
    //? Tamaño de las etiquetas de la grafica
    .x_label_area_size(20)
    .y_label_area_size(20)
    //? Construimos los limtes de la grafica
    .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

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

  //? Dibujamos la línea de la presion calculada
  chart.draw_series(LineSeries::new(
    conf
      .dp
      .iter()
      .zip(conf.pvnkt.iter())
      .map(|(&a, &b)| (a, b))
      .collect::<Vec<_>>(),
    Into::<ShapeStyle>::into(&PURPLE).stroke_width(2),
  ))?;
  //? Añadimos la linea de presion teorica por cs
  chart.draw_series(LineSeries::new(
    vec![
      (x_min + 0.01, conf.compress_cs),
      (x_max - 0.01, conf.compress_cs),
    ],
    Into::<ShapeStyle>::into(&RED).stroke_width(2),
  ))?;
  //? Añadimos la linea de presion teorica por bn
  chart.draw_series(LineSeries::new(
    vec![
      (x_min + 0.01, conf.compress_bn),
      (x_max - 0.01, conf.compress_bn),
    ],
    Into::<ShapeStyle>::into(&BLACK).stroke_width(2),
  ))?;

  //? Guardamos los cambios en la grafica
  root.present()?;
  //? Retornamos Ok si no hay errores
  Ok(())
}
