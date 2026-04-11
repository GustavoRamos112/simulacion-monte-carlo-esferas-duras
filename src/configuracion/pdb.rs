use std::fs::File;
use std::io::{Write, BufWriter};
use super::variables::Configuracion;

pub fn pdb(conf : &mut Configuracion, dir_pdb: &str, sigmar: &f64) {
  let file: File = File::create(format!("{}", dir_pdb))
    .expect("No se pudo crear el archivo");

  let mut file = BufWriter::new(file);

  let w1: &str = "ATOM";
  let w3: &str = "     ";
  let w4: &str = "    ";
  let n1: i32 = 1;

  //let sigmar: f64 = 3.405_f64;

  // equivalente a SYMBOL(I) = '  H'
  let symbol = "  H";

  for i in 0..conf.nfcc {

    conf.rxf = conf.rx[i] * sigmar;
    conf.ryf = conf.ry[i] * sigmar;
    conf.rzf = conf.rz[i] * sigmar;

    writeln!(
      file,
      "{:<4}{:>7}{:<4}{:<5}{:>2}{:>4}{:<4}{:8.3}{:8.3}{:8.3}",
      w1,
      i + 1,
      symbol,
      w3,
      n1,
      i + 1,
      w4,
      conf.rxf,
      conf.ryf,
      conf.rzf
    ).unwrap();
  }
}