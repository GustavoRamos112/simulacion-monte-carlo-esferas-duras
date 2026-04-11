use crate::configuracion::variables::Configuracion;

pub fn nrdf(conf: &mut Configuracion, sigma: f64) {

  conf.dp[0] = 0.0001;
  conf.dp[1] = 0.0025;
  conf.dp[2] = 0.005;
  conf.dp[3] = 0.0075;
  conf.dp[4] = 0.01;
  conf.dp[5] = 0.0125;
  conf.dp[6] = 0.015;
  conf.dp[7] = 0.0175;
  conf.dp[8] = 0.02;
  conf.dp[9] = 0.0225;
  conf.dp[10] = 0.025;
  conf.dp[11] = 0.0275;
  conf.dp[12] = 0.03;
  conf.dp[13] = 0.0325;
  conf.dp[14] = 0.035;

  //? Este es el constructor del vector nga y gr ya que 
  //? anteriormente se definia con memoria estatica:
  //?   nga: vec![0.0; 5000],
  //?   gr: vec![0.0; 5000],
  //? pero con estos pueden definirse dinamicamente
  //? Esto se justifica ya que en la funcion rdf original
  //? los valores de nga y gr se leian unicamente hasta el
  //? valor de nig el cual se calcula a partir
  //? de valores ya conocidos, por lo cual no depende
  //? explicitamente del orden de ejecucion de las funciones
  conf.nig = ((0.5*conf.boxx/conf.deltar) as usize) + 1;
  conf.nga = vec![0.0; conf.nig];
  conf.gr = vec![0.0; conf.nig];
  conf.r0 = vec![0.0; conf.nig];

  for k in 0..conf.l_press_dp {
    conf.sigmadp[k] = sigma + conf.dp[k];
  }

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
        if rij >= sigma && rij <= conf.sigmadp[k] {
          conf.sumpxx[k] += dx*dx/rij/conf.dp[k];
          conf.sumpyy[k] += dy*dy/rij/conf.dp[k];
          conf.sumpzz[k] += dz*dz/rij/conf.dp[k];
        }
      }
    }
  }
}