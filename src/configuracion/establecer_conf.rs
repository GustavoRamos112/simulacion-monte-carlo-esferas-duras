
use colored::Colorize;

use super::variables::Configuracion;
use super::fcc::fcc;
use super::pdb::pdb;


pub fn establecer_configuracion(
  mut conf: &mut Configuracion,
  color: &bool,
  dir_pdb: &str,
  sigmar: &f64
) {

  fcc(&mut conf);
  for i in 0..conf.nfcc {
      conf.rx[i] = conf.rxfcc[i];
      conf.ry[i] = conf.ryfcc[i];
      conf.rz[i] = conf.rzfcc[i];
  }
  let _ = pdb(&mut conf, dir_pdb, sigmar);

  if *color {
    println!( 
      "{}\n{}\n{}",
      "┌───────────────────────────┐".green(),
      "│ Configuracion establecida │".green(),
      "└───────────────────────────┘".green()
    );
  } else {
    println!(
      "{}\n{}\n{}",
      "┌───────────────────────────┐",
      "│ Configuracion establecida │",
      "└───────────────────────────┘"
    );
  }

}