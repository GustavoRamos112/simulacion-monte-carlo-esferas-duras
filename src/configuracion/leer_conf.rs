use serde::{Deserialize};
use std::fs;
use std::path::Path;
use colored::Colorize;

#[derive(Deserialize, Debug)]
pub struct ConfUsuario {
  pub nc: usize,
  pub dens: f64,
  pub titulo_sim: String,
  pub n_step: usize,
  pub intervalo_print: usize,
  pub i_ratio: usize,
  pub ngr: usize,
  pub sigmar: f64,
  pub generar_graficas: bool,
  pub color: bool,
  pub dir_graficas: String,
  pub dir_pdb: String,
  pub dir_dat: String,

  pub grap_gr_name: String,
  pub grap_presion_name: String,
  pub gr_dat_name: String,
  pub presion_dat_name: String,
  pub pdb_inicial: String,
  pub pdb_final: String,

  pub separador_datos: String,
  pub renderizar_pdb: bool,
}

pub fn cargar_conf_usuario() -> ConfUsuario {
  let data = fs::read_to_string("resources\\conf.json")
    .expect("No existe el archivo de configuracion");

  let mut conf_u: ConfUsuario = serde_json::from_str(&data)
    .expect("El json no esta formateado correctamente");

  if !Path::new(&conf_u.dir_graficas).exists() {
    match fs::create_dir_all(&conf_u.dir_graficas) {
      Ok(_) => {},
      Err(e) => {
        if conf_u.color {
          println!("No se pudo crear el directorio para graficas: {}", e.to_string().red());
        } else {
          println!("No se pudo crear el directorio para graficas: {}", e);
        }
        conf_u.dir_graficas = String::from(".\\");
      },
    }
  }
  if !Path::new(&conf_u.dir_pdb).exists() {
    match fs::create_dir_all(&conf_u.dir_pdb) {
      Ok(_) => {},
      Err(e) => {
        if conf_u.color {
          println!("No se pudo crear el directorio pdb: {}", e.to_string().red());
        } else {
          println!("No se pudo crear el directorio pdb: {}", e);
        }
        conf_u.dir_pdb = String::from(".\\");
      }
    }
  }
  if !Path::new(&conf_u.dir_dat).exists() {
    match fs::create_dir_all(&conf_u.dir_dat) {
      Ok(_) => {},
      Err(e) => {
        if conf_u.color {
          println!("No se pudo crear el directorio datos: {}", e.to_string().red());
        } else {
          println!("No se pudo crear el directorio datos: {}", e);
        }
        conf_u.dir_dat = String::from(".\\");
      }
    }
  }

  conf_u
}