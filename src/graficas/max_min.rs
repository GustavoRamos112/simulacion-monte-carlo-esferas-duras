//? Funcion que calcula el minimo y maximo de un vector de datos
pub fn min_max_f64(data: &Vec<f64>) -> Option<(f64, f64)> {
  //? Si el vector esta vacio, no se puede calcular el minimo y maximo
  if data.is_empty() {
    return None;
  }

  let (min, max) = data.iter().fold(
    (f64::INFINITY, f64::NEG_INFINITY),
    |(min_val, max_val), &curr| (min_val.min(curr), max_val.max(curr)),
  );

  Some((min, max))
}
