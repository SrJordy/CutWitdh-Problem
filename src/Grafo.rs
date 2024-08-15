use std::collections::{hash_set, HashSet}; //importamos la estructura hash_set para almacenar elementos de forma unica

//Definimos los tipos para almacenar los nodos y aristas
pub type Nodo=usize; //declaramos atributo nodo como tipo entero sin signo
pub type Arista=(Nodo,Nodo); //declaramos que el atributo arista sera una tupla que va a contener dos elementos

#[derive(Debug)] //esto es para que la estructura pueda ser formateada para impresión usando :? o :#? en macros como println!
pub struct Grafo{ //declaramos una estructura que tendra dos campos
    nodos:HashSet<Nodo>, //aqui en nodos avamos a almacenar los Nodo que teniamos antes pero en un hash_set para que no existan repetidos
    aristas:Vec<Arista> //aqui almacenamos en aristas un vector donde se guardaran las aristas que se declararon antes
}

impl Grafo{ //aqui establecemos un bloque impl que es para definir metodos que van a hacer uso de la estructura de grafo
    pub fn new(nodos:HashSet<Nodo>, aristas:Vec<Arista>)->Self{//aqui declaramos un constructor que toma un HashSet de nodos y un Vec de aristas, y devuelve una nueva instancia de Graph
        Self{nodos,aristas} // con self hacemos como remplazo del Grafo por lo que toma los nodos y aristas y las retorna como una nueva instancia de tipo Grafo
    }

    pub fn nodos(&self)->&HashSet<Nodo>{//declaramos un metodo el cual va a retornar una referencia al conjunto de nodos en el grafo
        &self.nodos //retornamos esas referencia a los nodos que estan almacenados en el campo nodos del grafo
        //self ya esta referenciado al grafo de arriba entonces cuando se utilice sera como estar declarando grafo.nodos para darle la referencia a los nodos
    }

    pub fn nodos_vec(&self)->Vec<Nodo>{//aqui lo que hacemos es crear un metodo que esta referenciado al grafo, de tal forma que va a retornar un vector con los nodos
        self.nodos.iter().cloned().collect() //aqui lo que hacemos es mediante self.nodos accedemos a los nodos que tiene el hash_set, luego recorremos eso con iteraciones y luego las clonamos
        //se debe hacer un clones ya que al momento de iterar estamos haciendo referencia al Nodo declarado arriba, pero con el cloned lo que se hace es convertir esa referencia a valor y luego de eso
        //se colecta todos los nodos clonados y los almacena en el vec<Nodo>
    }

    pub fn aristas_vec(&self)->&Vec<Arista>{//aqui declaramos un metodo que va a devolver una referencia al vector que almacena las aristas en el grafo
        &self.aristas //retornamos la referencia a las aristas almacenadas en el campo aristas de la estructura del grafo
    }
}