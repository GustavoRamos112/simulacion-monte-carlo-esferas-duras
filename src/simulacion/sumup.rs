use crate::configuracion::variables::Configuracion;

pub fn sumup(conf: &mut Configuracion) -> bool {
  for i in 0..conf.nfcc - 1 {
    let rxi: f64 = conf.rx[i];
    let ryi: f64 = conf.ry[i];
    let rzi: f64 = conf.rz[i];

    for j in i + 1..conf.nfcc {
      let mut rxij: f64 = rxi - conf.rx[j];
      let mut ryij: f64 = ryi - conf.ry[j];
      let mut rzij: f64 = rzi - conf.rz[j];

      rxij -= (rxij * conf.boxix).round() * conf.boxx;
      ryij -= (ryij * conf.boxiy).round() * conf.boxy;
      rzij -= (rzij * conf.boxiz).round() * conf.boxz;

      let rij: f64 = (rxij * rxij + ryij * ryij + rzij * rzij).sqrt();

      if rij < conf.sigma {
        return true;
      }
    }
  }

  false
}
