use rfd::FileDialog; // Importamos FileDialog para abrir una ventana de diálogo para seleccionar archivos
use crate::Cargar_Archivo::CargarArchivo; // Importamos la función CargarArchivo desde el módulo Cargar_Archivo
use crate::Heuristica::heuristica; // Importamos la estructura heuristica desde el módulo Heuristica
use std::time::Instant; // Importamos Instant para medir el tiempo de ejecución

mod Cargar_Archivo; // Declaramos el módulo Cargar_Archivo
mod Grafo; // Declaramos el módulo Grafo
mod Calcular_Cutwidth; // Declaramos el módulo Calcular_Cutwidth
mod Heuristica; // Declaramos el módulo Heuristica

fn main() {
    // Abrimos la ventana para seleccionar el archivo
    let ventana = FileDialog::new() // Creamos una nueva instancia de FileDialog
        .set_title("Seleccione un archivo de grafo") // Establecemos el título de la ventana de diálogo
        .add_filter("Archivo de texto", &["txt"]) // Añadimos un filtro para mostrar solo archivos de texto
        .pick_file() // Abrimos la ventana de diálogo para seleccionar un archivo
        .expect("Ocurrió un error al abrir el archivo"); // Mostramos un mensaje de error si no se selecciona un archivo

    // Seleccionamos el archivo
    let ruta = ventana // Obtenemos la ruta del archivo seleccionado
        .to_str() // Convertimos la ruta a una cadena de texto (string)
        .expect("Ocurrió un error al convertir la ruta a string"); // Mostramos un mensaje de error si falla la conversión

    println!("Archivo seleccionado: {}", ruta); // Imprimimos la ruta del archivo seleccionado

    // Cargamos el archivo a un grafo
    let grafo = CargarArchivo::Cargar(ruta).expect("Error al cargar el grafo");
    // Usamos la función Cargar para cargar el grafo desde el archivo y mostramos un error si falla

    // Creamos las instancias de la heurística
    let numero_soluciones = 100; // Establecemos el número total de soluciones a generar
    let muestra = 5; // Establecemos el tamaño de la muestra (10 soluciones por iteración)
    let numero_iteraciones = 4; // Establecemos el número de iteraciones (nuevas combinaciones por solución)
    let heur = heuristica::new(&grafo, numero_soluciones, muestra);
    // Creamos una nueva instancia de la heurística con los parámetros especificados

    // Medimos el tiempo de ejecución (esto es algo extra)
    let inicio = Instant::now(); // Guardamos el tiempo inicial para medir la duración de la ejecución

    // Buscamos la mejor solución después de realizar las iteraciones
    let (mejor_solucion_global, mejor_cuwi_global, mejor_corte_global, resultados_por_solucion) = heur.mejor_solucion(numero_iteraciones);

    let mut mejor_solucion_indice = 0; // Almacena el índice de la mejor solución global
    let mut mejor_suma_global = usize::MAX; // Almacena la mejor suma de cortes global



    // Luego mostramos los detalles de cada solución y sus iteraciones
    for (sol_idx, sumas_de_cortes, mejor_iteracion_indice, mejor_iteracion_corte) in &resultados_por_solucion {
        println!("\nSolución {}:", sol_idx + 1);
        for (iter, suma) in sumas_de_cortes.iter().enumerate() {
            println!("Iteración {}: Suma de cortes = {}", iter + 1, suma);
        }

        // Mostrar la mejor iteración de esta solución
        let max_corte = mejor_iteracion_corte.iter().max_by_key(|&&(_, _, size)| size).unwrap();
        println!("Mejor iteración: {} con un total de {} y su mayor corte es {} entre [{} , {}]",
                 mejor_iteracion_indice + 1,
                 sumas_de_cortes[*mejor_iteracion_indice],
                 max_corte.2, max_corte.0, max_corte.1
        );

        // Comparamos para encontrar la mejor solución global
        if sumas_de_cortes[*mejor_iteracion_indice] < mejor_suma_global {
            mejor_suma_global = sumas_de_cortes[*mejor_iteracion_indice];
            mejor_solucion_indice = *sol_idx;
        }
    }
    // Primero mostramos todas las soluciones con su mejor resultado
    println!("\nResultados de todas las soluciones:");
    for (sol_idx, sumas_de_cortes, mejor_iteracion_indice, _) in &resultados_por_solucion {
        println!("Solución {}: Total de la mejor iteración = {}", sol_idx + 1, sumas_de_cortes[*mejor_iteracion_indice]);
    }

    // Mostrar la mejor solución global en términos de la suma de cortes
    println!("\nLa mejor solución es la solución {} con un total de {}", mejor_solucion_indice + 1, mejor_suma_global);
    println!("Ordenamiento de la mejor solución global:");
    for nodo in mejor_solucion_global.iter() {
        print!("{}, ", nodo);
    }
    println!();

    // Mostrar el corte específico con el valor máximo de cutwidth en la mejor solución global
    if let Some((u, v, tamano_corte)) = mejor_corte_global.iter().max_by_key(|&&(_, _, tamano)| tamano) {
        println!("Corte con mayor valor de cutwidth: {} entre [{} , {}]", tamano_corte, u, v);
    }

    // Mostrar el tiempo de ejecución
    let tiempofin = inicio.elapsed();
    println!("La duración del cálculo fue de: {:.7} segundos", tiempofin.as_secs_f64());
}
