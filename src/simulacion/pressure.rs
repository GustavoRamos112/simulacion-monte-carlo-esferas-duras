use crate::configuracion::variables::Configuracion;

use std::fs::File;
use std::io::{BufWriter, Write};
use tabled::settings::Style;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct TableData {
  k: usize,
  dp: f64,
  zxx: f64,
  zyy: f64,
  zzz: f64,
  pvnkt: f64,
}

pub fn pressure(conf: &mut Configuracion, n_stepg: f64, dir_press_dat: &str, dat_sep: &str) {
  let file: File = File::create(dir_press_dat).expect("No se pudo crear el archivo");

  let mut file = BufWriter::new(file);

  let mut valores_tabla: Vec<TableData> = Vec::new();

  for k in 0..conf.l_press_dp {
    conf.sumpxx[k] /= n_stepg;
    conf.sumpyy[k] /= n_stepg;
    conf.sumpzz[k] /= n_stepg;

    let zxx: f64 = 1.0 + conf.sumpxx[k] / (conf.nfcc as f64);
    let zyy: f64 = 1.0 + conf.sumpyy[k] / (conf.nfcc as f64);
    let zzz: f64 = 1.0 + conf.sumpzz[k] / (conf.nfcc as f64);

    conf.pvnkt[k] = (zxx + zyy + zzz) / 3.0;

    valores_tabla.push(TableData {
      k: k,
      dp: conf.dp[k],
      zxx: zxx,
      zyy: zyy,
      zzz: zzz,
      pvnkt: conf.pvnkt[k],
    });

    writeln!(
      file,
      "{}{}{}{}{}{}{}",
      conf.dp[k], dat_sep, conf.compress_cs, dat_sep, conf.compress_bn, dat_sep, conf.pvnkt[k]
    )
    .unwrap();
  }

  let mut table = Table::new(valores_tabla);
  table.with(Style::modern());
  println!("{}", table);
}
