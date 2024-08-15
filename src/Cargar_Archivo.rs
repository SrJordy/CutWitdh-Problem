use std::fs::File; //importamos el modulo que permite manejar lo que son los archivos
use std::io::{BufRead,BufReader}; //aqui importamos los modulos que permitiran la lectura del archivo linea por linea
use std::collections::HashSet;//importamos el hashset para almacenar nodos unicos
use crate::Grafo::{Grafo,Nodo, Arista}; //importamos el grafo, nodo y arista de la clase Grafo

pub struct CargarArchivo; //declaramos una estructura vacia que se va a llamar CargarArchivo

impl CargarArchivo{//implementamos metodos a la estructura declarada
    pub fn Cargar(Ruta:&str)->std::io::Result<Grafo>{//Declaramos un metodo llamado cargar que va a apuntar a una cadena queseria la ruta la cual va a ser la que contenga el grafo
        //una vez cargado el grafo va a retornar el grafo o un error
        let archivo=File::open(Ruta)?; //Intentamos abrir el archivo especificado y obtenemos un objeto archivo
        let leer=BufReader::new(archivo); //Creamos un BufReader para leer el archivo de manera eficiente línea por línea
        let mut lineas=leer.lines(); //establecemos que la variable lineas va a ser mutable con el iterador lines del bufreader

        let primera_linea=lineas.next().unwrap()?; //leemos la primera linea del archivo para obtener los nodos y aristas, de tal forma que en caso de eque no existan mas lineas el unwrap finalice el proceso
        let mut partes=primera_linea.split_whitespace(); //dividimos la linea en partes usando espacios en blanco como un delimitador y el unwrap para saber si existen mas lineas
        let contar_nodos=partes.next().unwrap().parse::<usize>().unwrap(); //aqui lo que hacemos es leemos y parseamos los nodos haciendo uso del parse la cadena str la convierte en usize,
        // y el unwrap se asegura de que existe algo mas para continuar
        let contar_aristas=partes.next().unwrap().parse::<usize>().unwrap();

        let mut nodos=HashSet::new(); //declaramos una variable que sea mutable como hashset para almacenar los nodos de forma unica
        let mut aristas:Vec<Arista>=Vec::new(); //declaramos un vector de tipo Vec<Arista> para almacenar las aristas

        for linea in lineas { //empezamos a iterar en las lineas restantes del archivo
            let linea=linea?; //leemos cada linea, propagando errores si los hay
            let mut partes=linea.split_whitespace();//Dividimos la línea en partes usando espacios en blanco como delimitadores
            let u=partes.next().unwrap().parse::<Nodo>().unwrap(); //leemos y parseamos el primer nodo de la arista
            let v=partes.next().unwrap().parse::<Nodo>().unwrap();//leemos y parseamos el segundo nodo de la arista
            nodos.insert(u);//insertamos el primer nodo en el Hashset
            nodos.insert(v);//insertamos el segundo nodo en el hashset
            aristas.push(Arista::from((u,v)));//usamos la Arista que importamos para añadir la arista u,v en el vector aristas
        }
        Ok(Grafo::new(nodos,aristas))//Retornamos el grafo pero con los valores del archivo cargado
    }
}