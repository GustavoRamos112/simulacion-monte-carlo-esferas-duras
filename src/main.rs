use colored::Colorize;
use std::path::PathBuf;
use tabled::{builder::Builder, settings::Style};

//? Modulos creados
mod configuracion;
use configuracion::establecer_conf::establecer_configuracion;
use configuracion::leer_conf;
use configuracion::variables::Configuracion;

mod simulacion;
use simulacion::ejecutar_simulacion::ejecutar_simulacion;

mod graficas;
use graficas::g_gr::g_gr;
use graficas::g_presion::g_presion;

mod pdb_view;
use pdb_view::view_pdb::view_pdb;

fn main() {
    //? Se leen los parametros desde un archivo de configuracion
    let conf_u: leer_conf::ConfUsuario = leer_conf::cargar_conf_usuario();
    let mut conf: Configuracion = Configuracion::new(conf_u.nc, conf_u.dens);
    //? Establecemos la configuracion de la simulacion
    let pdb_inicial_path = PathBuf::from(&conf_u.dir_pdb).join(&conf_u.pdb_inicial);
    let _ = establecer_configuracion(
        &mut conf,
        &conf_u.color,
        pdb_inicial_path.to_str().unwrap(),
        &conf_u.sigmar,
    );
    //? Imprimimos las instrucciones
    instrucciones(&conf_u.color);
    //? Se ejecuta el programa interactivo
    loop {
        //? Leemos la opcion escogida
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
            //? En caso que la opcion sea > 5 o no sea valida
            0_u8 => {
                println!("Instruccion no valida");
                instrucciones(&conf_u.color);
            }
            //? Iniciamos la simulacion
            1_u8 => {
                ejecutar_simulacion_cof_usr(&mut conf, &conf_u);
            }
            //? Generamos las graficas
            2_u8 => {
                generar_graficas_pasadas(&mut conf, &conf_u);
            }
            //? Visualizamos el PDB inicial
            3_u8 => {
                let pdb_path = PathBuf::from(&conf_u.dir_pdb).join(&conf_u.pdb_inicial);
                view_pdb_pasado(&mut conf, &conf_u.sigmar, pdb_path.to_str().unwrap());
            }
            //? Visualizamos el PDB final
            4_u8 => {
                let pdb_path = PathBuf::from(&conf_u.dir_pdb).join(&conf_u.pdb_final);
                view_pdb_pasado(&mut conf, &conf_u.sigmar, pdb_path.to_str().unwrap());
            }
            //? Sale del programa
            5_u8 => {
                std::process::exit(0);
            }
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

fn ejecutar_simulacion_cof_usr(conf_i: &mut Configuracion, conf_u_i: &leer_conf::ConfUsuario) {
    //? Iniciamos la simulacion
    let mut conf = conf_i;
    let conf_u = conf_u_i;
    ejecutar_simulacion(&mut conf, &conf_u);
    if conf_u.generar_graficas {
        //? Graficas
        let grap_print: Vec<&str> = vec![
            "┌────────────────────┐",
            "│ Generando graficas │",
            "└────────────────────┘",
        ];

        for i in grap_print {
            if conf_u.color {
                println!("{}", i.yellow());
            } else {
                println!("{}", i);
            }
        }

        let presion_path = PathBuf::from(&conf_u.dir_graficas).join(&conf_u.grap_presion_name);
        let gr_path = PathBuf::from(&conf_u.dir_graficas).join(&conf_u.grap_gr_name);
        let _ = g_presion(&conf, presion_path.to_str().unwrap()).unwrap();
        let _ = g_gr(&conf, gr_path.to_str().unwrap()).unwrap();

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
                "┌─────────────────────┐", "│ Graficas  generadas │", "└─────────────────────┘"
            );
        }
    }
    //? Visualizamos el PDB final
    if conf_u.renderizar_pdb {
        view_pdb(&conf, &conf_u.sigmar, &true);
    }
}

//? Funcion que permite generar graficas con
//? los ficheros de datos de G(R) y presion
//? definidos en el json
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
            "┌────────────────────┐", "│ Generando graficas │", "└────────────────────┘"
        );
    }

    //? Comprobamos que existan los archivos de datos (g(r))
    let mut dir = PathBuf::from(&conf_u.dir_dat).join(&conf_u.gr_dat_name);
    let mut dir_interno = dir.clone();
    let mut i: usize = 0;
    match std::fs::exists(dir) {
        //? Si existe, leemos los datos
        Ok(_) => {
            let push_vec: bool = conf.r0.len() == 0;

            for line_result in std::fs::read_to_string(&dir_interno).unwrap().lines() {
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

            let gr_path = PathBuf::from(&conf_u.dir_graficas).join(&conf_u.grap_gr_name);
            let _ = g_gr(&conf, gr_path.to_str().unwrap()).unwrap();
        }
        //? Si no, imprimimos un error
        Err(_) => {
            println!("No se encontro el archivo de datos de gr");
            println!(
                "Ejecute una simulacion o importe los archivos de datos a la ruta configurada"
            );
        }
    }

    //? Lo mismo para el archivo de datos de presion
    dir = PathBuf::from(&conf_u.dir_dat).join(&conf_u.presion_dat_name);
    dir_interno = dir.clone();
    i = 0;
    match std::fs::exists(&dir) {
        Ok(_) => {
            for line_result in std::fs::read_to_string(&dir_interno).unwrap().lines() {
                let line = line_result.trim();
                let datos: Vec<&str> = line.split(&conf_u.separador_datos).collect();

                conf.dp[i] = datos[0].parse::<f64>().unwrap();
                conf.compress_cs = datos[1].parse::<f64>().unwrap();
                conf.compress_bn = datos[2].parse::<f64>().unwrap();
                conf.pvnkt[i] = datos[3].parse::<f64>().unwrap();
                i += 1;
            }

            let presion_path = PathBuf::from(&conf_u.dir_graficas).join(&conf_u.grap_presion_name);
            let _ = g_presion(&conf, presion_path.to_str().unwrap()).unwrap();
        }
        Err(_) => {
            println!("No se encontro el archivo de datos de presion");
            println!(
                "Ejecute una simulacion o importe los archivos de datos a la ruta configurada"
            );
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
            "┌─────────────────────┐", "│ Graficas  generadas │", "└─────────────────────┘"
        );
    }
}

//? Funcion que permite visualizar el pdb desde un archivo pdb
fn view_pdb_pasado(conf: &mut Configuracion, sigmar: &f64, pdb_dir: &str) {
    let dir_interno = pdb_dir;
    //? Comprobamos que exista el archivo pdb
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
        }
        Err(_) => {
            println!("No se encontro el archivo pdb");
            println!("Ejecute una simulacion o importe el archivo pdb a la ruta configurada");
        }
    }
}
