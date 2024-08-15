use rfd::FileDialog;
use crate::Cargar_Archivo::CargarArchivo;
use crate::Heuristica::heuristica;
use std::time::Instant;
mod Cargar_Archivo;
mod Grafo;
mod Calcular_Cutwidth;
mod Heuristica;

fn main() {
    //abrimos la ventana para seleccionar el archivo
    let ventana=FileDialog::new()
        .set_title("Seleccione un archivo de grafo")
        .add_filter("Archivo de texto",&["txt"])
        .pick_file()
        .expect("Ocurrio un error al abrir el archivo");
    //seleccionamos el archivo
    let ruta=ventana
        .to_str()
        .expect("Ocurrio un error al convertir la ruta a string");

    println!("Archivo seleccionado: {}",ruta);

    //cargamos el archivo a un grafo
    let grafo=CargarArchivo::Cargar(ruta).expect("Error al cargar el grafo");

    //creamos las instancias de la heuristica
    let numero_soluciones=1000;
    let muestra=10;
    let heur=heuristica::new(&grafo, numero_soluciones, muestra);

    //medimos el tiempo de ejecucion esto es algo extra
    let inicio=Instant::now();

    //Buscamos la mejor solucion
    let (mejor_solu, cuwi_max, mejor_corte)=heur.mejor_solucion();

    //mostramos le mejor ordenacion que se pudo encontrar el en cuwi
    println!("Esta es la mejor ordenacion encontrada: ");
    for nodo in mejor_solu.iter() {
        print!("{}, ", nodo);
    }
    println!();

    //buscamos ahora el corte con el valor maximo de cuwi
    if let Some((u, v, tamaño_corte))=mejor_corte.iter().find(|&&(_,_,tamaño)|tamaño==cuwi_max){
        println!("Corte de: {} entre [{},{}]",tamaño_corte,u,v);
    }

    let tiempofin=inicio.elapsed();
    println!("La duracion del cuwi fue de: {:.7} segundos", tiempofin.as_secs_f64());
}
