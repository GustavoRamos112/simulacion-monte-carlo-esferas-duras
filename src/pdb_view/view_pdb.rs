#[warn(unused)]

use raylib::prelude::*;
use crate::configuracion::variables::Configuracion;

pub fn view_pdb(conf: &Configuracion, sigmar: &f64, multiplicar: &bool) {
  let (mut rl, thread) = raylib::init()
    .size(800, 600)
    .title("PDB Viewer")
    .build();
  rl.disable_cursor();
  rl.set_target_fps(60);

  let l = ((conf.boxx / 2.0) * sigmar) as f32;

  let mut camera = Camera3D::perspective(
    Vector3::new(0.0, 0.0, l * 3.0),
    Vector3::new(0.0, 0.0, 0.0),
    Vector3::new(0.0, 1.0, 0.0),
    45.0,
  );

  while !rl.window_should_close() {
    rl.update_camera(&mut camera, CameraMode::CAMERA_FREE);

    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::BLACK);

    let mut d3 = d.begin_mode3D(camera);

    // 🔹 dibujar caja
    d3.draw_cube_wires(
      Vector3::new(0.0, 0.0, 0.0),
      l * 2.0,
      l * 2.0,
      l * 2.0,
      Color::BLUE,
    );

    // 🔹 dibujar partículas
    for i in 0..conf.nfcc {
      let mut pos = Vector3::new(0.0, 0.0, 0.0);
      if *multiplicar {
        pos = Vector3::new(
          (conf.rx[i] * sigmar) as f32,
          (conf.ry[i] * sigmar) as f32,
          (conf.rz[i] * sigmar) as f32,
        );
      } else {
        pos = Vector3::new(
          (conf.rx[i]) as f32,
          (conf.ry[i]) as f32,
          (conf.rz[i]) as f32,
        );
      }

      d3.draw_sphere(pos, 0.1, Color::GRAY);
    }
  }
}