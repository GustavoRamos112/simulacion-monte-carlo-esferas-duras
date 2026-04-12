use crate::configuracion::variables::Configuracion;

pub fn nrdf(conf: &mut Configuracion) {
  /* conf.nig = ((0.5*conf.boxx/conf.deltar) as usize) + 1;
  conf.nga = vec![0.0; conf.nig];
  conf.gr = vec![0.0; conf.nig];
  conf.r0 = vec![0.0; conf.nig]; 
  
  for k in 0..conf.l_press_dp {
    conf.sigmadp[k] = sigma + conf.dp[k];
  }
  */

  for i in 0..conf.nfcc-1 {
    let rxi: f64 = conf.rx[i];
    let ryi: f64 = conf.ry[i];
    let rzi: f64 = conf.rz[i];

    for j in i+1..conf.nfcc {
      let mut dx: f64 = rxi - conf.rx[j];
      let mut dy: f64 = ryi - conf.ry[j];
      let mut dz: f64 = rzi - conf.rz[j];

      dx -= (dx*conf.boxix).round()*conf.boxx;
      dy -= (dy*conf.boxiy).round()*conf.boxy;
      dz -= (dz*conf.boxiz).round()*conf.boxz;

      let rij: f64 = (dx*dx + dy*dy + dz*dz).sqrt();

      if rij <= 0.5*conf.boxx {
        let irij: f64 = rij/conf.deltar;
        //? Ejecutamos esta linea por precaucion
        //? (aunque si hay un desbordameinto, el
        //? compilador de rust lo indicara)
        if irij < conf.nig as f64 {
          conf.nga[irij as usize] += 2.0;
        }
      }

      for k in 0..conf.l_press_dp {
        if rij >= conf.sigma && rij <= conf.sigmadp[k] {
          conf.sumpxx[k] += dx*dx/rij/conf.dp[k];
          conf.sumpyy[k] += dy*dy/rij/conf.dp[k];
          conf.sumpzz[k] += dz*dz/rij/conf.dp[k];
        }
      }
    }
  }
}