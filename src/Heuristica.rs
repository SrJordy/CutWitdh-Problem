use std::collections::{hash_set, HashSet}; //importamos la estructura hash_set para almacenar elementos de forma unica

//Definimos los tipos para almacenar los nodos y aristas
pub type Nodo=usize; //declaramos nodo como tipo entero sin signo
pub type Arista=(Nodo,Nodo); //declaramos que arista sera una tupla que va a contener dos elementos

#[derive(Debug)] //esto es para que la estructura pueda ser formateada para impresión usando :? o :#? en macros como println!
pub struct Grafo{ //declaramos una estructura que tendra dos campos
    nodos:hash_set<Nodo>, //aqui en nodos avamos a almacenar los Nodo que teniamos antes pero en un hash_set para que no existan repetidos
    aristas:Vec<Arista> //aqui almacenamos en aristas un vector donde se guardaran las aristas que se declararon antes
}

impl Grafo{ //aqui establecemos un bloque impl que es para definir metodos que van a hacer uso de la estructura de grafo
    pub fn new(nodos:HashSet<Nodo>, aristas:Vec<Arista>)->Self{//aqui declaramos un constructor que toma un HashSet de nodos y un Vec de aristas, y devuelve una nueva instancia de Graph
        Self{nodos,aristas}
    }
}