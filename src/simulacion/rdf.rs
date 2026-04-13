use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::configuracion::variables::Configuracion;

pub fn rdf(conf: &mut Configuracion, n_stepg: f64, dir_gr_dat: &str, dat_sep: &str) {
  let file: File = File::create(dir_gr_dat).expect("No se pudo crear el archivo");

  let mut file = BufWriter::new(file);

  let rhoj: f64 = (conf.nfcc as f64) / conf.volumen;
  let denx: f64 = (n_stepg as f64) * (conf.nfcc as f64);
  let fact: f64 = 4.0 * PI * rhoj / 3.0;

  for jj in 0..conf.nig {
    conf.r0[jj] = (jj as f64) * conf.deltar;
    let r: f64 = conf.r0[jj] + conf.deltar;
    let den: f64 = fact * (r.powi(3) - conf.r0[jj].powi(3));
    let gar: f64 = conf.nga[jj] / den;
    conf.gr[jj] = gar / denx;
    writeln!(file, "{}{}{}", conf.r0[jj], dat_sep, conf.gr[jj]).unwrap();
  }
}
