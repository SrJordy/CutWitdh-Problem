use rfd::FileDialog;
use crate::Cargar_Archivo::CargarArchivo;
use crate::Heuristica::heuristica;
use std::time::Instant;

mod Cargar_Archivo;
mod Grafo;
mod Calcular_Cutwidth;
mod Heuristica;

fn main() {
    // Abrimos la ventana para seleccionar el archivo
    let ventana = FileDialog::new()
        .set_title("Seleccione un archivo de grafo")
        .add_filter("Archivo de texto", &["txt"])
        .pick_file()
        .expect("Ocurrió un error al abrir el archivo");

    // Seleccionamos el archivo
    let ruta = ventana
        .to_str()
        .expect("Ocurrió un error al convertir la ruta a string");

    println!("Archivo seleccionado: {}", ruta);

    // Cargamos el archivo a un grafo
    let grafo = CargarArchivo::Cargar(ruta).expect("Error al cargar el grafo");

    // Creamos las instancias de la heurística
    let numero_soluciones = 10000;
    let muestra = 5;
    let heur = heuristica::new(&grafo, numero_soluciones, muestra);

    // Medimos el tiempo de ejecución (esto es algo extra)
    let inicio = Instant::now();

    // Buscamos la mejor solución
    let (mejor_solu, cuwi_max, mejor_corte, suma_de_cortes, mejor_indice) = heur.mejor_solucion();

    // Mostramos las cantidades de soluciones en base a la muestra
    println!("Suma de cortes para cada solución en la muestra:");
    for (i, suma) in suma_de_cortes.iter().enumerate() {
        println!("Solución {}: {}", i + 1, suma);
    }

    // Mostramos qué solución fue la mejor en términos de suma mínima de cortes
    println!("\nLa mejor solución encontrada fue la solución {}.", mejor_indice + 1);

    // Mostramos la mejor ordenación encontrada con el mínimo cuwi
    println!("Ordenación correspondiente: ");
    for nodo in mejor_solu.iter() {
        print!("{}, ", nodo);
    }
    println!();

    // Identificamos el corte con el mayor valor de cutwidth en la mejor solución
    if let Some((u, v, max_cuwi)) = mejor_corte.iter().max_by_key(|&&(_, _, tamaño)| tamaño) {
        println!("Corte con mayor valor de cutwidth: {} entre [{} , {}]", max_cuwi, u, v);
    }

    let tiempofin = inicio.elapsed();
    println!("La duración del cuwi fue de: {:.7} segundos", tiempofin.as_secs_f64());
}
