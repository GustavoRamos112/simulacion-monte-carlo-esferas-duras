use tabled::{builder::Builder, settings::Style};

use crate::configuracion::variables::Configuracion;

fn imprimir_tabla(eta: f64, den_0: f64, density_0: f64, conf: &Configuracion) {
    let mut builder = Builder::new();
    builder.push_record(["ETA", eta.to_string().as_str()]);
    builder.push_record(["RHO/RHO_0", (den_0 / density_0).to_string().as_str()]);
    builder.push_record(["PV/NKT(CS)", conf.compress_cs.to_string().as_str()]);
    builder.push_record(["PV/NKT(BN)", conf.compress_bn.to_string().as_str()]);

    let mut table = builder.build();
    table.with(Style::ascii_rounded());

    println!("{}", table);
}

pub fn valores_teoricos(conf: &mut Configuracion) {
    let eta: f64 = std::f64::consts::PI * conf.dens / 6.0;

    let sum_0: f64 = 1.0 + eta + eta * eta;
    let den_0: f64 = (1.0 - eta).powi(3);
    conf.compress_cs = (sum_0 - eta.powi(3)) / den_0;

    let density_0: f64 = 2.0_f64.sqrt();

    let b1: f64 = 0.764314;
    let b2: f64 = 0.151532;
    let b3: f64 = 0.654551;

    let sum_1: f64 = b1 * eta.powi(3) + b2 * eta.powi(4) + b3 * eta.powi(5);
    conf.compress_bn = (sum_0 - sum_1) / den_0;

    imprimir_tabla(eta, den_0, density_0, conf);
}
