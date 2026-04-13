use super::variables::Configuracion;

pub fn fcc(conf: &mut Configuracion) {
  conf.boxx = (conf.nfcc as f64 / conf.dens).powf(1.0 / 3.0);
  conf.boxy = conf.boxx;
  conf.boxz = conf.boxx;

  conf.boxix = 1.0 / conf.boxx;
  conf.boxiy = 1.0 / conf.boxy;
  conf.boxiz = 1.0 / conf.boxz;

  //println!("boxx {}", conf.boxx);

  conf.cellx = conf.boxx / conf.nc as f64;
  conf.celly = conf.boxy / conf.nc as f64;
  conf.cellz = conf.boxz / conf.nc as f64;
  conf.cell2x = 0.5 * conf.cellx;
  conf.cell2y = 0.5 * conf.celly;
  conf.cell2z = 0.5 * conf.cellz;

  conf.rxfcc[0] = 0.0;
  conf.ryfcc[0] = 0.0;
  conf.rzfcc[0] = 0.0;

  conf.rxfcc[1] = conf.cell2x;
  conf.ryfcc[1] = conf.cell2y;
  conf.rzfcc[1] = 0.0;

  conf.rxfcc[2] = 0.0;
  conf.ryfcc[2] = conf.cell2y;
  conf.rzfcc[2] = conf.cell2z;

  conf.rxfcc[3] = conf.cell2x;
  conf.ryfcc[3] = 0.0;
  conf.rzfcc[3] = conf.cell2z;
  let mut m: usize = 0;
  for iz in 0..conf.nc {
    for iy in 0..conf.nc {
      for ix in 0..conf.nc {
        for iref in 0..4 {
          conf.rxfcc[iref + m] = conf.rxfcc[iref] + conf.cellx * (ix as f64);
          conf.ryfcc[iref + m] = conf.ryfcc[iref] + conf.celly * (iy as f64);
          conf.rzfcc[iref + m] = conf.rzfcc[iref] + conf.cellz * (iz as f64);
        }
        m += 4;
      }
    }
  }
}
