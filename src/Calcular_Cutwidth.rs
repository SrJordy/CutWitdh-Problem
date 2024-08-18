use std::cmp::Ordering;
use crate::Grafo::{Grafo, Nodo};//importamos lo que es el grafo y nodo de la clase Grafo
use std::collections::HashMap;//importamos hasmap para mapear todos los nodos a posiciones en la ordenacion

pub struct CalcularCutwidth<'a>{//establecemos una estrucura con <'a> para establecer un tiempo de vida para las referencias (muere la referencia osea el grafo y muere el proceso de calculo)
    grafo: &'a Grafo //Definimos la estructura CalcularCutwidth que contiene una referencia al grafo
}

impl <'a> CalcularCutwidth <'a> {
    pub fn new(grafo: &'a Grafo)->Self{//Creamos un metodo que crea una nueva instancia de calcularcuwi
        Self{grafo}//retornarmos la instancia pero con referncia al grafo
    }

    pub fn Calcular(&self, ordering: &[Nodo])->(usize, Vec<(Nodo,Nodo,usize)>){ //con este metodo se va a calcular el cuwi basandose en la ordenacion de los nodos
        let mut cuwi_maximo=0;//declaramos una variable que va a almacenar el valor maximo encontrado del cuwi
        let mut cortes=Vec::new();//declaramos un vector para almacenar los detalles de cada corte, nodos y el tamaño del corte

        let posicion: HashMap<_,_>=ordering.iter().enumerate().map(|(idx,nodo)|(nodo,idx)).collect(); //lo que hacemos es mapear la ordenacion de los nodos para
        //iteraro, mediante enumerate lo que hace es tomar las iteraciones y darles un indice el cual va a ser aprovechado por el map ya que se guardaria indice, nodo y el map lo que hace es invertir eso
        //luego con el collect lo que hace es que toma todo eso lo colecciona y lo convierte en hashmap

        //Esto facilita la identificación de la posición relativa de cada nodo en la lista ordenada

        for i in 0..ordering.len()-1 { //empezamos a recorrer todas las posiciones de corte en la ordenacion
            let mut corte_actual=0; //declaramos una variable que puede ser mutable para almacenar el corte actual
            for &(ref u, ref v) in self.grafo.aristas_vec() {//hacemos un recorrido por todas las aristas y contamos cuantas cruzan en corte
                let posi_u=posicion[u]; //obtenemos la posicion del nodo U
                let posi_v=posicion[v];// obtenemos la posicion del nodo V

                if (posi_u<=i && posi_v>i)|| (posi_v<=i && posi_u>i) {//si la arista cruza el corte entre las posiciones i e i+1, incrementa el contador de cutwidth
                    corte_actual+=1;
                }
            }
            if corte_actual>cuwi_maximo {//si el cuwi actual es mayor al cuwi maximo registrado lo que se hace es reemplazar
                cuwi_maximo=corte_actual;
            }

            cortes.push((ordering[i].clone(), ordering[i+1].clone(), corte_actual));//en el vector de cortes almacenamos los detalles, nodo, nodo y tamaño de corte

        }
        (cuwi_maximo,cortes)//retorna el cuwi maximo y el detalle de los cortes
    }
}