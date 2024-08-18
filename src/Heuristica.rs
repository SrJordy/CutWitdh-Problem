use crate::Grafo::{Nodo, Grafo}; // Importamos Nodo y Grafo desde el módulo Grafo
use crate::Calcular_Cutwidth::CalcularCutwidth; // Importamos CalcularCutwidth desde el módulo Calcular_Cutwidth
use rand::seq::SliceRandom; // Importamos SliceRandom para mezclar el orden de los nodos de forma aleatoria
use rand::Rng; // Importamos Rng para la generación de números aleatorios

pub struct heuristica<'a> { // Definimos la estructura heuristica con un lifetime para la referencia al grafo
    grafo: &'a Grafo, // Referencia al grafo sobre el cual se aplicará la heurística
    tamaño_muestra: usize, // Tamaño de la muestra para comparar las combinaciones
    numero_soluciones: usize // Número de soluciones aleatorias a generar
}

impl<'a> heuristica<'a> {
    pub fn new(grafo: &'a Grafo, numero_soluciones: usize, tamaño_muestra: usize) -> Self {
        // Constructor para crear una nueva instancia de heuristica
        Self {
            grafo, numero_soluciones, tamaño_muestra,
            // Inicializamos el grafo, el número de soluciones y el tamaño de la muestra
        }
    }

    pub fn generar_solucion(&self) -> Vec<Vec<Nodo>> {
        // Método que genera un conjunto de soluciones aleatorias (combinaciones de nodos)
        let mut random = rand::thread_rng(); // Generador de números aleatorios
        let mut solucion = Vec::new(); // Vector para almacenar las soluciones generadas

        let nodos: Vec<Nodo> = self.grafo.nodos().iter().cloned().collect();
        // Clonamos los nodos del grafo en un vector

        while solucion.len() < self.numero_soluciones {
            // Generamos nuevas soluciones hasta alcanzar el número deseado
            let mut orden = nodos.clone(); // Clonamos el vector de nodos para crear una nueva ordenación
            orden.shuffle(&mut random); // Mezclamos de forma aleatoria el orden usando el generador aleatorio
            solucion.push(orden); // Agregamos la nueva ordenación al vector de soluciones
        }
        solucion // Retornamos el vector de soluciones generadas
    }

    pub fn mejor_solucion(&self, num_iteraciones: usize) -> (Vec<Nodo>, usize, Vec<(Nodo, Nodo, usize)>, Vec<(usize, Vec<usize>, usize, Vec<(Nodo, Nodo, usize)>)>) {
        // Encuentra la mejor solución (menor cutwidth) después de realizar un número dado de iteraciones por solución
        let mut mejor_solucion_global = Vec::new(); // Vector para almacenar la mejor ordenación encontrada globalmente
        let mut mejor_cuwi_global = usize::MAX; // Inicializamos el cutwidth mínimo global como el valor más alto posible
        let mut mejor_corte_global = Vec::new(); // Vector para almacenar los detalles del mejor corte de la mejor solución global
        let mut resultados_por_solucion = Vec::new(); // Almacena los resultados de las iteraciones por solución

        let soluciones = self.generar_solucion(); // Generamos las soluciones aleatorias iniciales
        let calcular_cuwi = CalcularCutwidth::new(self.grafo); // Instanciamos CalcularCutwidth para calcular el cutwidth

        // Iteramos sobre cada solución en la muestra
        for (sol_idx, solucion) in soluciones.into_iter().enumerate().take(self.tamaño_muestra) {
            let mut mejor_cuwi_iteracion = usize::MAX; // Inicializamos el mejor cutwidth para las iteraciones de esta solución
            let mut mejor_iteracion_corte = Vec::new(); // Almacena el mejor corte para la mejor iteración de esta solución
            let mut mejor_iteracion_indice = 0; // Almacena el índice de la mejor iteración
            let mut sumas_de_cortes = Vec::new(); // Almacena las sumas de cortes de cada iteración

            // Realizamos las iteraciones y generamos nuevas combinaciones
            for iter in 0..num_iteraciones {
                let mut nueva_solucion = solucion.clone(); // Clonamos la solución original
                let mut rng = rand::thread_rng();
                let i = rng.gen_range(0..nueva_solucion.len());
                let j = rng.gen_range(0..nueva_solucion.len());
                nueva_solucion.swap(i, j); // Intercambiamos dos nodos aleatorios

                let (cuwi, cortes) = calcular_cuwi.Calcular(&nueva_solucion); // Calculamos el cutwidth de la nueva combinación
                let suma_cortes: usize = cortes.iter().map(|(_, _, size)| size).sum(); // Calculamos la suma de cortes
                sumas_de_cortes.push(suma_cortes); // Guardamos la suma de cortes para esta iteración

                // Si encontramos una mejor suma de cortes en esta iteración, actualizamos la mejor
                if suma_cortes < mejor_cuwi_iteracion {
                    mejor_cuwi_iteracion = suma_cortes; // Actualizamos la mejor suma de cortes para esta solución
                    mejor_iteracion_corte = cortes.clone(); // Actualizamos los detalles del corte
                    mejor_iteracion_indice = iter; // Guardamos el índice de la mejor iteración
                }

                // Si esta iteración tiene el mejor cutwidth global, la guardamos
                if cuwi < mejor_cuwi_global {
                    mejor_cuwi_global = cuwi;
                    mejor_solucion_global = nueva_solucion.clone();
                    mejor_corte_global = cortes.clone();
                }
            }

            // Guardamos los resultados de esta solución
            resultados_por_solucion.push((sol_idx, sumas_de_cortes, mejor_iteracion_indice, mejor_iteracion_corte));
        }

        // Devolvemos la mejor solución global, el mejor cutwidth global, los detalles del corte global y los resultados por solución
        (mejor_solucion_global, mejor_cuwi_global, mejor_corte_global, resultados_por_solucion)
    }
}
