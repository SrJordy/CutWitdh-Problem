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

    pub fn mejor_solucion(&self) -> (Vec<Nodo>, usize, Vec<(Nodo, Nodo, usize)>, Vec<usize>, usize) {
        // Encuentra la mejor solución (menor cuwi) y calcula las sumas de cortes de las soluciones
        let mut mejor_solucion = Vec::new(); // Vector para almacenar la mejor ordenación encontrada
        let mut mejor_cuwi_sum = usize::MAX; // Inicializamos la suma mínima del cuwi como el valor más alto posible
        let mut mejor_corte = Vec::new(); // Vector para almacenar los detalles de los cortes de la mejor solución
        let mut suma_de_cortes = Vec::new(); // Vector para almacenar las sumas de cortes de todas las soluciones en la muestra
        let mut mejor_indice = 0; // Para rastrear el índice de la mejor solución

        let solucion = self.generar_solucion(); // Generamos las soluciones aleatorias
        let calcular_cuwi = CalcularCutwidth::new(self.grafo); // Instanciamos CalcularCutwidth para calcular el cuwi

        let muestra = solucion.choose_multiple(&mut rand::thread_rng(), self.tamaño_muestra);
        // Seleccionamos una muestra aleatoria de las soluciones generadas para evaluar

        for (i,orden) in muestra.enumerate() { // Iteramos sobre cada orden en la muestra
            let (cuwi, cortes) = calcular_cuwi.Calcular(orden);

            let suma_cortes: usize = cortes.iter().map(|(_, _, size)| size).sum();
            // Calculamos la suma de todos los cortes para esta ordenación

            suma_de_cortes.push(suma_cortes); // Almacenamos la suma de los cortes para esta ordenación

            if suma_cortes < mejor_cuwi_sum {
                // Si la suma de cortes es menor que la mejor suma registrada...
                mejor_solucion = orden.clone(); // Clonamos la mejor ordenación encontrada
                mejor_cuwi_sum = suma_cortes; // Actualizamos la mejor suma de cortes
                mejor_corte = cortes; // Actualizamos los detalles de los cortes
                mejor_indice = i; // Almacena el índice de la mejor solución
            }
        }

        // Devolvemos la mejor ordenación, la suma mínima de cortes, los detalles de los cortes y todas las sumas de cortes
        (mejor_solucion, mejor_cuwi_sum, mejor_corte, suma_de_cortes, mejor_indice)
    }
}
