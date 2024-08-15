use crate::Grafo::{Nodo, Grafo}; //importamos el nodo y el grafo de la clase grafo
use crate::Calcular_Cutwidth::CalcularCutwidth;//importamos  el calcular cuwi
use rand::seq::SliceRandom; //importamos slicerandom para poder mezclar el orden de los nodos de forma aleatoria
use rand::Rng;//importamos Rng para generar numeros aleatorios

pub struct heuristica<'a>{//creamos una estructura llamada heuristica la cual va a tener un tiempo de vida segun la referencia
    grafo: &'a Grafo,//referenciamos el grafo donde se aplicara la heuristica
    tamaño_muestra:usize, //tamaño de la muestra para comparar las combinaciones
    numero_soluciones:usize //numero de soluciones aleatorias a generar
}

impl <'a> heuristica<'a>{
    pub fn new(grafo: &'a Grafo, numero_soluciones:usize, tamaño_muestra:usize)->Self{//este es el constructor para crear una nueva instancia de la heuristica
        Self{
            grafo,numero_soluciones,tamaño_muestra, //aqui inicializamosel numero de soluciones, el grafo y el tamaño de la muestra
        }
    }

    pub fn generar_solucion(&self)->Vec<Vec<Nodo>>{//este metodo se encargar de generar las solucionesaleatoria o combinaciones de los nodos
        let mut random=rand::thread_rng();//creamos una variable que generara los numeros randoms
        let mut solucion=Vec::new();//inicializamos un vector vacio para almacenar las soluciones generadas

        let nodos:Vec<Nodo>=self.grafo.nodos().iter().cloned().collect();//creamos una variable de tipo vector a partir de los nodos del grafo, clonando cada nodo

        while solucion.len()<self.numero_soluciones {//se van a generar un numero de soluciones hasta alcanzar el deseado
            let mut orden=nodos.clone();//clonamos el vector de nodos para crear una nueva ordenacion
            orden.shuffle(&mut random);//mezclamos de forma aleatoria el orden usando el random
            solucion.push(orden);//agregamos el nuevo orden al vector de solucion
        }
        solucion //retornamos el vector de soluciones generadas
    }

    pub fn mejor_solucion(&self)->(Vec<Nodo>, usize, Vec<(Nodo,Nodo,usize)>){//con este metodo se va a encontrar la mejor solucion del cuwi osea el menor de las generadas
        let mut mejor_solucion=Vec::new();//inicializamos el vector para poder almacenar la mejor ordenacion encontrada
        let mut mejor_cuwi=usize::MAX;//inicializamos el cuwi maximo como el valor mas alto posible
        let mut mejor_corte=Vec::new();//inicializamos un vector para poder almacenar los detalles de los cortes de la mejor solucion

        let solucion=self.generar_solucion();//generamos la solucion aleatoria establecida anteriormente

        let calcular_cuwi=CalcularCutwidth::new(self.grafo);//creamos una instancia de la clase cuwi para poder calcular el cuwi de las ordenaciones
        let muestra=solucion.choose_multiple(&mut rand::thread_rng(), self.tamaño_muestra); //seleccionamos una muestra aleatoria de las soluciones generadas para evaluar

        for orden in muestra{ //vamos a iterar cada orden en la muestra
            let (cuwi, cortes)=calcular_cuwi.Calcular(orden);
            if cuwi<mejor_cuwi {
                mejor_solucion=orden.clone();//clonamos porque el orden hace referencia a Vec<Nodo>
                mejor_cuwi=cuwi;
                mejor_corte=cortes;
            }
        }
        (mejor_solucion, mejor_cuwi, mejor_corte)
    }
}