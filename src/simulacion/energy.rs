use crate::configuracion::variables::Configuracion;

pub fn energy (
  conf: &mut Configuracion, 
  rxi: f64, ryi: f64, rzi: f64, 
  i: usize
) -> bool 
{
  for j in 0..conf.nfcc {
    if i == j { continue }
    let mut rxij: f64 = rxi - conf.rx[j];
    let mut ryij: f64 = ryi - conf.ry[j];
    let mut rzij: f64 = rzi - conf.rz[j];

    rxij -= (rxij*conf.boxix).round()*conf.boxx;
    ryij -= (ryij*conf.boxiy).round()*conf.boxy;
    rzij -= (rzij*conf.boxiz).round()*conf.boxz;

    let rij: f64 = (rxij*rxij + ryij*ryij + rzij*rzij).sqrt();

    if rij < conf.sigma {
      return true;
    }
  }
  return false;
}