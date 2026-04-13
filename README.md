# Simulacion de Monte Carlo para esferas duras

## Primera parte: Conceptos basicos

### ¿Que es montecarlo?
Una simulacion Montecarlo es una tecnica de simulacion que estima los posibles resultados de un evento mediante la repeticion de eventos aleatorios.

### ¿Que es una cadena de Markov?
Un modelo matematico estocastico que describe una secuencia de eventos donde la probabilidad del siguiente estado depende unicamente del estado actual, no de la historia de eventos.

### ¿Que es el algoritmo de Metropolis?
Metodo Montecarlo de cadena de Markov que genera una secuencia de muestras aleatorias a partir de una distribucion de probabilidad la cual es dificil muestrear directamente.

### ¿Esferas duras?
Se considera una esfera dura a una esfera con un radio sigma que se encuentra en un punto determinado en el espacio, es impenetrable, no ejerce fuerza a otras esferas a la distacia y tampoco puede solaparse con otra.

## Segunda parte: Explicacion del modelo

### Potencial
En esferas duras no hay energia potencial continua; si hay solapamiento entre esferas, el potencial es infinito, si no, es cero.

### Simulacion
#### 1: Delimitamos una caja
Consideramos una caja en el espacio donde encerramos el sistema de esferas duras.
#### 2: Configuracion inicial
Para cada esfera, asignamos un punto inicial dentro de la caja y despues verificamos que no haya solapamiento entre las esferas.
#### 3: Actualizacion de posiciones
Se toma una esfera de la configuracion y se mueve a lo largo de los tres ejes de forma aleatoria. Se comprueba que no haya solapamiento y en caso de no, se actualiza la posicion de la esfera, en caso contrario, tomamos otra esfera y se repite el proceso. (Otros factores detallados de forma mas detallada en la siguiente parte)

## Tercera parte: Codigo
### Codigo general
#### Requisitos
Para la ejecucion de este codigo se usaran los paquetes:

- [fastrand](https://docs.rs/fastrand/latest/fastrand/) - Generacion de numeros aleatorios
- [plotters](https://docs.rs/plotters/latest/plotters/) - Generacion de graficas
- [raylib](https://docs.rs/raylib/latest/raylib/) - Visualizacion 3D de archivos PDB
- [serde](https://serde.rs/) - Serializacion de configuracion JSON
- [tabled](https://docs.rs/tabled/latest/tabled/) - Tablas en consola
- [colored](https://docs.rs/colored/latest/colored/) - Colores en terminal
#### Capacidades del codigo
El codigo presenta una tabla de comandos:
```` bash
╭───┬───────────────────────────────────╮
│ 1 │ Iniciar simulacion de Monte Carlo │
├───┼───────────────────────────────────┤
│ 2 │ Generar graficas                  │
├───┼───────────────────────────────────┤
│ 3 │ Ver PDB inicial                   │
├───┼───────────────────────────────────┤
│ 4 │ Ver PDB final                     │
├───┼───────────────────────────────────┤
│ 5 │ Salir                             │
╰───┴───────────────────────────────────╯
````

1. Inicia la simulacion de Montecarlo tomando la configuracion antes definida en el json (simulacion detallada en la siguiente seccion)
2. genera las graficas del archivo de datos de G(R) y presion definidos en el json (Puede graficar archivos de datos generados no necesariamente por este programa, solo hay que definir el separador de los datos, al direccion y el nombre del/los archivo(s) a graficar)
3. Visualiza el pdb inicial (configuracion inicial)
4. Visualiza el pdb final (De igual forma, el codigo es capaz de visualizar archivos de pdb generados no necesariamente por este programa, solo hay que definir el nombre del archivo a visualizar y su direccion)
5. Sale del programa (sin necesidad de un Ctrl+C)



### Simulacion computacional

#### I - Configuracion
Por comodidad, estas variables se estableceran en un json (resources\conf.json) que se lee y se almacena en una estructura de tipo `ConfUsuario` (src\configuracion\leer_conf.rs)

Todos los parametros de simulacion se configuran en `resources/conf.json`:

```json
{
  "nc": 3,                    // Numero de repeticiones en cada direccion (n_particulas = nc^3)
  "dens": 0.5,                // Densidad adimensional del fluido
  "titulo_sim": "HS",         // Titulo de la simulacion
  "n_step": 5000,             // Numero de pasos de la simulacion
  "intervalo_print": 500,     // Intervalo de impresion de progreso
  "i_ratio": 5,               // Intervalo de actualizacion del radio de aceptacion
  "ngr": 100,                 // Intervalo para calculo de g(r)
  "sigmar": 3.405,            // Factor de escala para visualizar PDB
  "generar_graficas": true,   // Generar graficas al finalizar
  "color": true,              // Mostrar colores en consola
  "renderizar_pdb": false,    // Visualizar PDB al finalizar
  "dir_graficas": "archives/graficas", // Directorio donde se guardan las graficas
  "dir_pdb": "archives/pdb", // Directorio donde se guardan los .pdb
  "dir_dat": "archives/datos", // Directorio donde se guardan los .dat
  "grap_gr_name": "gr.png", // Nombre de la grafica de G(R)
  "grap_presion_name": "presion.png", // Nombre de la grafica de presion
  "gr_dat_name": "gr.dat", // Nombre del archivo de datos de G(R)
  "presion_dat_name": "presion.dat", // Nombre del archivo de datos de presion
  "pdb_inicial": "FotoInicial.pdb", // Nombre del pdb inicial
  "pdb_final": "FotoFinal.pdb", // Nombre del pdb final
  "separador_datos": ", " // Separador de los datos en los archivos de datos <recomendado " ,">
}
```

#### II - Configuracion
Se establecen todas las variables que usaremos en la simulacion en una estructura de tipo `Configuracion` (src\configuracion\variables.rs) dentro de este archivo se encuentra la explicacion de cada variable.

Se genera la configuracion inicial y se genera el pdb respectivo.

#### III - Cadena de Markov
Se hace uso de un bucle for cuya estructura general es:
```` rust
for i_step in 1..=conf_u.n_step {
  // 1. Intentos de mover todas las partículas (un "ciclo" o "sweep")
  for i in 0..conf.nfcc {
      // mover partícula i
  }
  // 2. Ajuste de drmax
  // 3. Imprimir estado de la simulación
  // 4. Cálculo de g(r)
}
````

Para mover una particula, hacemos uso de
```` rust
let mut rx_i_new: f64 = rx_i_old + (2.0 * fastrand::f64() - 1.0) * drmax;
````

se verifican que las particulas no se muevan fuera de la caja
```` rust
rx_i_new -= (rx_i_new*conf.boxix).round()*conf.boxx;
````

En este caso, guardamos el valor en una nueva variable para despues hacer uso de la funcion `energy` (src\simulacion\energy.rs) para verificar si hay o no solapamiento con otra particula.

```` rust
if energy(
  &mut conf, 
  rx_i_new, ry_i_new, rz_i_new,
  i, sigma
) { continue }
````
En caso de solapamiento, la posicion de la particula no se actualiza y se pasa a la siguiente. 

En caso contrario, se actualiza la posicion, se suma 1 a la variable acmmva (cantidad de movimientos aceptados) y se pasa a la siguiente particula.
```` rust
conf.rx[i] = rx_i_new;
conf.ry[i] = ry_i_new;
conf.rz[i] = rz_i_new;
acmmva += 1.0;
````

Poseriormente se ajusta drmax, donde ratio es la fracción de aceptación en ese bloque: movimientos aceptados dividido por el número total de intentos (nfcc * i_ratio).

Objetivo: mantener ratio cerca de 0.5 (50% de aceptación).

Si ratio > 0.5 → los pasos son demasiado pequeños (se acepta mucho) → aumentamos drmax un 5%.

Si ratio < 0.5 → los pasos son demasiado grandes (muchos rechazos) → disminuimos drmax un 5%.
```` rust
if i_step % conf_u.i_ratio == 0 {
  ratio = acmmva / ((conf.nfcc * conf_u.i_ratio) as f64);
  if ratio > 0.5 {
      drmax *= 1.05;
  } else {
      drmax *= 0.95;
  }
  acmmva = 0.0;
}
````

Imprimimos el progreso de la simulacion:
```` rust
if i_step % conf_u.intervalo_print == 0 {
  println!("{}\t{}", i_step, ratio);
}
````

Finalmente, si i_step es un multiplo de ngr, se llama a nrdf (src\simulacion\nrdf.rs) para acumular histogramas de g(r).
```` rust
if i_step % conf_u.ngr == 0 {
  nrdf(&mut conf, sigma);
}
````

Despues del for principal, ejecutamos las funciones para exportar los datos de presion y G(R) a archivos de datos (src\simulacion\pressure.rs y src\simulacion\rdf.rs respectivamente).

#### IV - Visualizacion y graficas
Esta secion no sera tan detallada.

Para visualizar el pdb, se usa el codigo de rust de [raylib-rs](https://docs.rs/raylib/latest/raylib/) (src\pdb_view\view_pdb.rs).

Para generar graficas, se usa el codigo de rust de [plotters](https://docs.rs/plotters/latest/plotters/) (src\graficas\g_gr.rs y src\graficas\g_presion.rs).

---

## Cuarta parte: Instalacion y compilacion

### Requisitos del sistema
- **Rust** 1.70+ ([instalar](https://rustup.rs/))
- **CMake** (para compilar dependencias nativas de raylib)

#### Windows:
- MSVC v14.3 - VS 2022 C++ x64/x86 build tools (https://visualstudio.microsoft.com/es/downloads/)

#### Linux:
```` bash
sudo apt install build-essential git
````
Para las dependencias de raylib, visitar: https://github.com/raysan5/raylib/wiki/Working-on-GNU-Linux


### Compilacion

```bash
# Clonar el repositorio
git clone https://github.com/GustavoRamos112/simulacion-monte-carlo-esferas-duras.git
cd simulacion-monte-carlo-esferas-duras

# Compilar en modo debug
cargo build

# Compilar en modo release (recomendado para simulaciones)
cargo build --release

# Ejecutar
cargo run --release
```

---

## Quinta parte: Resultados

### Archivos de salida

La simulacion genera los siguientes archivos:

| Archivo | Descripcion |
|---------|-------------|
| `*.pdb` | Configuracion atomica en formato PDB |
| `gr.dat` | Datos de la funcion de distribucion radial g(r) |
| `presion.dat` | Datos de presion y factor de compresibilidad |
| `gr.png` | Grafica de g(r) |
| `presion.png` | Grafica de presion |

### Parametros fisicos calculados

- **g(r)**: Funcion de distribucion radial
- **P**: Presion del sistema
- **Z**: Factor de compresibilidad (PV/NkT)
- **Cs**: Factor de compresibilidad isotropico

---

## Sexta parte: Licencia

<div align="center">

[![GNU GPLv3 Image](https://www.gnu.org/graphics/gplv3-127x51.png)](http://www.gnu.org/licenses/gpl-3.0.en.html)

</div>

<div align="left">

Puedes copiar, distribuir y modificar el software siempre que registres los cambios y las fechas en los archivos fuente. Cualquier modificación del software, incluido el código con licencia GPL (mediante compilación), también debe estar disponible bajo la licencia GPL, junto con las instrucciones de compilación e instalación.

</div>
