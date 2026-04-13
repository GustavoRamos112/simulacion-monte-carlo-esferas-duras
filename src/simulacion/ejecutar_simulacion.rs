use fastrand;
use tabled::{builder::Builder, settings::Style};

use colored::Colorize;

use crate::configuracion::leer_conf::ConfUsuario;
use crate::configuracion::pdb::pdb;
use crate::configuracion::variables::Configuracion;
use crate::simulacion::energy::energy;
use crate::simulacion::equation::valores_teoricos;
use crate::simulacion::nrdf::nrdf;
use crate::simulacion::pressure::pressure;
use crate::simulacion::rdf::rdf;
use crate::simulacion::sumup::sumup;

fn imprimir_tabla(conf: &Configuracion, conf_u: &ConfUsuario) {
  let mut builder = Builder::new();
  builder.push_record(["Titulo de la simulacion", conf_u.titulo_sim.as_str()]);
  builder.push_record(["Numero de atomos", conf.nfcc.to_string().as_str()]);
  builder.push_record(["Densidad adimensional", conf.dens.to_string().as_str()]);
  builder.push_record(["Volumen de la caja", conf.volumen.to_string().as_str()]);
  builder.push_record(["Numero de ciclos", conf_u.n_step.to_string().as_str()]);
  builder.push_record([
    "Intervalo de impresion",
    conf_u.intervalo_print.to_string().as_str(),
  ]);
  builder.push_record([
    "Ratio de actualizacion",
    conf_u.i_ratio.to_string().as_str(),
  ]);
  builder.push_record([
    "Frecuencia de calcular G(R)",
    conf_u.ngr.to_string().as_str(),
  ]);

  let mut table = builder.build();
  table.with(Style::modern_rounded());

  println!("{}", table);
}

pub fn ejecutar_simulacion(mut conf: &mut Configuracion, conf_u: &ConfUsuario) {
  if conf_u.color {
    println!(
      "{}\n{}\n{}\n{}",
      ".......................................".yellow(),
      ": Iniciando simulacion de Monte Carlo :".yellow(),
      ": para esferas duras                  :".yellow(),
      ".......................................".yellow()
    )
  } else {
    println!(
      "{}\n{}\n{}\n{}",
      ".......................................",
      ": Iniciando simulacion de Monte Carlo :",
      ": para esferas duras                  :",
      "......................................."
    )
  }

  //let sigma: f64 = 1.0;
  let mut drmax: f64 = 0.15;

  conf.volumen = conf.boxx * conf.boxy * conf.boxz;

  imprimir_tabla(&conf, &conf_u);

  if sumup(&mut conf) {
    eprintln!("Overlap en la configuracion inicial");
    std::process::exit(1);
  }

  let mut acmmva = 0.0;

  println!("Empezando la cadena de Markov");
  println!("Paso\tRadio");
  //? Bucle for principal
  for i_step in 1..=conf_u.n_step {
    //? Bucle for para mover cada particula
    for i in 0..conf.nfcc {
      //? Movemos la aprticula aleatoriamente
      let rx_i_old: f64 = conf.rx[i];
      let ry_i_old: f64 = conf.ry[i];
      let rz_i_old: f64 = conf.rz[i];

      let mut rx_i_new: f64 = rx_i_old + (2.0 * fastrand::f64() - 1.0) * drmax;
      let mut ry_i_new: f64 = ry_i_old + (2.0 * fastrand::f64() - 1.0) * drmax;
      let mut rz_i_new: f64 = rz_i_old + (2.0 * fastrand::f64() - 1.0) * drmax;
      //? Aseguramos que la particula no se mueva fuera de la caja
      rx_i_new -= (rx_i_new * conf.boxix).round() * conf.boxx;
      ry_i_new -= (ry_i_new * conf.boxiy).round() * conf.boxy;
      rz_i_new -= (rz_i_new * conf.boxiz).round() * conf.boxz;
      //? Verificamos si hay solapamiento
      if energy(&mut conf, rx_i_new, ry_i_new, rz_i_new, i) {
        continue;
      }
      //? Si no, actualizamos la posicion
      conf.rx[i] = rx_i_new;
      conf.ry[i] = ry_i_new;
      conf.rz[i] = rz_i_new;
      acmmva += 1.0;
    }
    //? Ajustamos drmax
    let mut ratio: f64 = 0.0;
    //? Si es el paso actual multiplo de i_step
    //? ajustamos drmax
    if i_step % conf_u.i_ratio == 0 {
      ratio = acmmva / ((conf.nfcc * conf_u.i_ratio) as f64);
      if ratio > 0.5 {
        drmax *= 1.05;
      } else {
        drmax *= 0.95;
      }
      acmmva = 0.0;
    }

    if i_step % conf_u.intervalo_print == 0 {
      println!("{}\t{}", i_step, ratio);
    }

    if i_step % conf_u.ngr == 0 {
      nrdf(&mut conf);
      //println!("Ejecutando nrdf")
    }
  }

  valores_teoricos(&mut conf);
  let n_stepg: f64 = conf_u.n_step as f64 / conf_u.ngr as f64;
  pressure(
    &mut conf,
    n_stepg,
    format!("{}\\{}", conf_u.dir_dat, conf_u.presion_dat_name).as_str(),
    &conf_u.separador_datos,
  );

  if n_stepg > 0.0 {
    rdf(
      &mut conf,
      n_stepg,
      format!("{}\\{}", conf_u.dir_dat, conf_u.gr_dat_name).as_str(),
      &conf_u.separador_datos,
    );
  }

  pdb(
    &mut conf,
    format!("{}\\{}", conf_u.dir_pdb, conf_u.pdb_final).as_str(),
    &conf_u.sigmar,
  );
  if conf_u.color {
    println!(
      "\n{}\n{}\n{}",
      "┌───────────────────────┐".green(),
      "│ Simulacion finalizada │".green(),
      "└───────────────────────┘".green()
    )
  } else {
    println!(
      "\n{}\n{}\n{}",
      "┌───────────────────────┐", "│ Simulacion finalizada │", "└───────────────────────┘"
    )
  }
}
