
#[derive(Clone)]
pub struct Configuracion
{
  //? Numero de repeticiones en cada direccion
  pub nc: usize,
  //? Densidad adimensional del fluido
  pub dens: f64,
  //? Numero total de particulas
  pub nfcc: usize,

  pub sigma: f64,

  //? tamaño de los vectores nga y gr
  //? Ver funcion establecer_configuracion 
  //? (src\configuracion\establecer_conf.rs)
  pub nig: usize,

  //? tamaño de los vectores:
  //? dp, sigmadp, sumpxx, sumpyy, sumpzz, pvnkt
  //? indica la cantidad de veces que se 
  //? calculara la presion en el sistema
  pub l_press_dp: usize,
  
  //? Vectores de coordenadas de las particulas
  pub rx: Vec<f64>,
  pub ry: Vec<f64>,
  pub rz: Vec<f64>,

  //? Vectores fcc (face centered cubic)
  //? Es el vector de coordenadas de las particulas
  //? para la configuracion inicial
  pub rxfcc: Vec<f64>,
  pub ryfcc: Vec<f64>,
  pub rzfcc: Vec<f64>,

  //? Dimensiones del sistema
  //? Para una caja cubica:
  //? boxx = boxy = boxz
  pub boxx: f64,
  pub boxy: f64,
  pub boxz: f64,

  //? Volumen del sistema
  pub volumen: f64,

  //? Inversos de las dimensiones del sistema
  pub boxix: f64,
  pub boxiy: f64,
  pub boxiz: f64,

  //? Variables auxiliares para definir la geometria
  //? de red FCC dentro de la caja
  pub cellx: f64,
  pub celly: f64,
  pub cellz: f64,
  pub cell2x: f64,
  pub cell2y: f64,
  pub cell2z: f64,

  //? Variables auxiliares para exportar el .pdb
  //* Dado que solo se usan en una ocacion,
  //* posiblemnte seran eliminadas en el futuro
  pub rxf: f64,
  pub ryf: f64,
  pub rzf: f64,

  //? Vectores de G(R) y R0
  pub nga: Vec<f64>,
  pub gr: Vec<f64>,
  pub r0: Vec<f64>,

  //? Vectores de presion
  pub dp: Vec<f64>,
  pub sigmadp: Vec<f64>,

  //? Vectores de suma de presion
  pub sumpxx: Vec<f64>,
  pub sumpyy: Vec<f64>,
  pub sumpzz: Vec<f64>,

  //? Presiones teoricas
  pub compress_cs: f64,
  pub compress_bn: f64,
  //? Vector de la presion de la simulacion
  pub pvnkt: Vec<f64>,
  //? Radio de aceptacion
  pub deltar: f64,
}

impl Configuracion
{
  pub fn new(nc: usize, dens: f64) -> Self {
    let nfcc: usize = 4 * nc.pow(3);
    let l_press_dp: usize = 15;
    let sigma: f64 = 1.0;
    Self {      
      nc, dens,
      nfcc, l_press_dp, sigma,
      nig: 0,

      rx: vec![0.0; nfcc],
      ry: vec![0.0; nfcc],
      rz: vec![0.0; nfcc],
      
      rxfcc: vec![0.0; nfcc],
      ryfcc: vec![0.0; nfcc],
      rzfcc: vec![0.0; nfcc],

      boxx: 0.0,
      boxy: 0.0,
      boxz: 0.0,

      volumen: 0.0,

      boxix: 0.0,
      boxiy: 0.0,
      boxiz: 0.0,

      cellx: 0.0,
      celly: 0.0,
      cellz: 0.0,
      cell2x: 0.0,
      cell2y: 0.0,
      cell2z: 0.0,

      rxf: 0.0,
      ryf: 0.0,
      rzf: 0.0,

      nga: vec![0.0],
      gr: vec![],
      r0: vec![],

      dp: vec![0.0; l_press_dp],
      sigmadp: vec![0.0; l_press_dp],

      sumpxx: vec![0.0; l_press_dp],
      sumpyy: vec![0.0; l_press_dp],
      sumpzz: vec![0.0; l_press_dp],

      compress_cs: 0.0,
      compress_bn: 0.0,
      pvnkt: vec![0.0; l_press_dp],

      deltar: 0.02,
    }
  }
}