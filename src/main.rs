use colored::Colorize;
use tabled::{builder::Builder, settings::Style};

//? Modulos creados
mod configuracion;
use configuracion::variables::Configuracion;
use configuracion::leer_conf;
use configuracion::establecer_conf::establecer_configuracion;

mod simulacion;
use simulacion::ejecutar_simulacion::ejecutar_simulacion;

mod graficas;
use graficas::g_presion::g_presion;
use graficas::g_gr::g_gr;

mod pdb_view;
use pdb_view::view_pdb::view_pdb;

fn main() {
  //? Se leen los parametros desde un archivo de configuracion
  let conf_u: leer_conf::ConfUsuario = leer_conf::cargar_conf_usuario();
  //? Llamamos a la funcion establecer_configuracion
  let mut conf: Configuracion = Configuracion::new(conf_u.nc, conf_u.dens);
  let _ = establecer_configuracion(
    &mut conf, &conf_u.color, 
    format!("{}\\{}", conf_u.dir_pdb, conf_u.pdb_inicial).as_str(), 
    &conf_u.sigmar
  );
  
  instrucciones(&conf_u.color);
  loop {
    let mut opcion: String = String::new();
    println!("Opcion: ");
    std::io::stdin()
      .read_line(&mut opcion)
      .expect("Error al leer opcion");

    let opcion_i: u8 = match opcion.trim().parse::<u8>() {
      Ok(valor) => valor,
      Err(_) => 0_u8,
    };

    match opcion_i {
      0_u8 => {
        println!("Instruccion no valida");
        //instrucciones(&conf_u.color);
      },
      1_u8 => {
        //? Iniciamos la simulacion
        ejecutar_simulacion_cof_usr(&mut conf, &conf_u);
      },
      2_u8 => {
        generar_graficas_pasadas(&mut conf, &conf_u);
      },
      3_u8 => {
        //? generamos el PDB inicial
        view_pdb_pasado(
          &mut conf, 
          &conf_u.sigmar,
          format!("{}\\{}", conf_u.dir_pdb, conf_u.pdb_inicial).as_str()
        );
      },
      4_u8 => {
        //? generamos el PDB final
        view_pdb_pasado(
          &mut conf, 
          &conf_u.sigmar,
          format!("{}\\{}", conf_u.dir_pdb, conf_u.pdb_final).as_str()
        );
      },
      5_u8 => {
        std::process::exit(0);
      },
      _ => {
        println!("Instruccion no valida (1)");
      }
    }
  }
}

fn instrucciones(color: &bool) {
  let mut builder = Builder::new();
  builder.push_record(["1", "Iniciar simulacion de Monte Carlo"]);
  builder.push_record(["2", "Generar graficas"]);
  builder.push_record(["3", "Ver PDB inicial"]);
  builder.push_record(["4", "Ver PDB final"]);
  builder.push_record(["5", "Salir"]);

  let mut table = builder.build();
  table.with(Style::modern_rounded());

  if *color {
    println!("{}", table.to_string().magenta());
  } else {
    println!("{}", table);
  } 

}

fn ejecutar_simulacion_cof_usr( conf_i: &mut Configuracion, conf_u_i: &leer_conf::ConfUsuario) {
  //? Iniciamos la simulacion
  let mut conf = conf_i;
  let conf_u = conf_u_i;
  ejecutar_simulacion(
    &mut conf, 
    &conf_u
  );
  if conf_u.generar_graficas {
    //? Graficas
    if conf_u.color {
      println!(
        "{}\n{}\n{}",
        "┌────────────────────┐".yellow(),
        "│ Generando graficas │".yellow(),
        "└────────────────────┘".yellow()
      );
    } else {
      println!(
        "{}\n{}\n{}",
        "┌────────────────────┐",
        "│ Generando graficas │",
        "└────────────────────┘"
      );
    }
    let _ = g_presion(
      &conf, 
      format!("{}\\{}", conf_u.dir_graficas, conf_u.grap_presion_name).as_str()
    ).unwrap();
    let _ = g_gr(
      &conf, 
      format!("{}\\{}", conf_u.dir_graficas, conf_u.grap_gr_name).as_str()
    ).unwrap();

    if conf_u.color {
      println!(
        "{}\n{}\n{}",
        "┌─────────────────────┐".green(),
        "│ Graficas  generadas │".green(),
        "└─────────────────────┘".green()
      );
    } else {
      println!(
        "{}\n{}\n{}",
        "┌─────────────────────┐",
        "│ Graficas  generadas │",
        "└─────────────────────┘"
      );
    }
  }

  if conf_u.renderizar_pdb {
    view_pdb(&conf, &conf_u.sigmar, &true);
  }
}

fn generar_graficas_pasadas(conf: &mut Configuracion, conf_u: &leer_conf::ConfUsuario) {
  if conf_u.color {
    println!(
      "{}\n{}\n{}",
      "┌────────────────────┐".yellow(),
      "│ Generando graficas │".yellow(),
      "└────────────────────┘".yellow()
    );
  } else {
    println!(
      "{}\n{}\n{}",
      "┌────────────────────┐",
      "│ Generando graficas │",
      "└────────────────────┘"
    );
  }

  let mut dir = format!("{}\\{}", conf_u.dir_dat, conf_u.gr_dat_name);
  let mut dir_interno = dir.clone();
  let mut i: usize = 0;
  match std::fs::exists(dir) {
    Ok(_) => {  
      let push_vec: bool = conf.r0.len() == 0;

      for line_result in std::fs::read_to_string(dir_interno).unwrap().lines() {
        let line = line_result.trim();
        let datos: Vec<&str> = line.split(&conf_u.separador_datos).collect();
        if push_vec {
          conf.r0.push(datos[0].parse::<f64>().unwrap());
          conf.gr.push(datos[1].parse::<f64>().unwrap());
        } else {
          conf.r0[i] = datos[0].parse::<f64>().unwrap();
          conf.gr[i] = datos[1].parse::<f64>().unwrap();
          i += 1;
        }
      }

      let _ = g_gr(
        &conf, 
        format!("{}\\{}", conf_u.dir_graficas, conf_u.grap_gr_name).as_str()
      ).unwrap();
    },
    Err(_) => {
      println!("No se encontro el archivo de datos de gr");
      println!("Ejecute una simulacion o importe los archivos de datos a la ruta configurada");
    }
  }

  dir = format!("{}/{}", conf_u.dir_dat, conf_u.presion_dat_name);
  dir_interno = dir.clone();
  i = 0;
  match std::fs::exists(dir) {
    Ok(_) => {  
      for line_result in std::fs::read_to_string(dir_interno).unwrap().lines() {
        let line = line_result.trim();
        let datos: Vec<&str> = line.split(&conf_u.separador_datos).collect();

        conf.dp[i] = datos[0].parse::<f64>().unwrap();
        conf.compress_cs = datos[1].parse::<f64>().unwrap();
        conf.compress_bn = datos[2].parse::<f64>().unwrap();
        conf.pvnkt[i] = datos[3].parse::<f64>().unwrap();
        i += 1;
      }

      let _ = g_presion(
        &conf, 
        format!("{}\\{}", conf_u.dir_graficas, conf_u.grap_presion_name).as_str()
      ).unwrap();
    },
    Err(_) => {
      println!("No se encontro el archivo de datos de presion");
      println!("Ejecute una simulacion o importe los archivos de datos a la ruta configurada");
    }
  }
  if conf_u.color {
    println!(
      "{}\n{}\n{}",
      "┌─────────────────────┐".green(),
      "│ Graficas  generadas │".green(),
      "└─────────────────────┘".green()
    );
  } else {
    println!(
      "{}\n{}\n{}",
      "┌─────────────────────┐",
      "│ Graficas  generadas │",
      "└─────────────────────┘"
    );
  }
}

fn view_pdb_pasado(conf: &mut Configuracion, sigmar: &f64, pdb_dir: &str) {
  let dir_interno = pdb_dir;
  match std::fs::exists(pdb_dir) {
    Ok(_) => {
      let mut i: usize = 0;
      for line_result in std::fs::read_to_string(dir_interno).unwrap().lines() {
        let line = line_result.trim();
        let data: Vec<&str> = line.split_whitespace().collect();

        conf.rx[i] = data[5].parse::<f64>().unwrap();
        conf.ry[i] = data[6].parse::<f64>().unwrap();
        conf.rz[i] = data[7].parse::<f64>().unwrap();
        i += 1;
      }

      view_pdb(&conf, sigmar, &false);
    },
    Err(_) => {
      println!("No se encontro el archivo pdb");
      println!("Ejecute una simulacion o importe el archivo pdb a la ruta configurada");
    }
  }
}