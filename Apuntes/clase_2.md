# Clase 2 -


## ¿Por qué Ownership?

Para eliminar categorías completa de bugs (punteros nulos, uso despues de liberación, liberacion doble, buffer overruns)


- Como surgen estos bugs?
    - Aliasing: Mas de un camino para alcanzar la misma memoria
    - Mutacion: Cambiar memoria en comun


En rust, cada valor tiene una variable que es su owner

Solo puede existir un owner a la vez

Cuando el owner sale de alcance, el valor sera liberado

Cada valor tiene un unico owner, que determine su lifetime, cuando su owner es liberado (droppped), el valor
owned tambien

Una variable es owner de su valor.

Se puede usar el sistema de tipos para validar algo que sea mutable o compartido, pero no ambas a la vez

&T -> Referencia compartida
&mut T -> Referencia mutable


---
un Box es un puntero a un valor de tipo T, almacenado en el heap.
Asigna espacio en heap, mueve el valor v a ese espacio, retorna un Box que apunta a ese espacio
Un box es owner del espacio al que apunta, de tal manera que cuando el Box es destruido, se libera su espacio tambien
---

Se puede llevar un conteo de referencias con RC y ARC. Esto les permite tener multiples owners, bajo ciertas restricciones

Para la mayoria de operaciones, no se copia un valor, sino que se mueven.


Referencias

Las referencias en rust son otro tipo de puntero que no son owners del valor al que apunta, sino que lo toman prestado
"borrow"

Las referencias en si, no son mas que direcciones de memoria.

Referencia compartida, varios la pueden leer, nadie puede escribirlo.
Referencia mutable, se puede leer y escribir, pero no puede haber mas de una referencia activa

Las referencias en rust nunca son nulas.
    No existe un valor por defecto inicial para una referencia
    Rust no convierte enteros en referencias
    Para indicar la ausencia de un valor, se usa Option<T>



Rust intenta asignar a cada tipo de referencia en un programa un lifetime que satisface las restricciones impuestas
por como se usa.

Un life es un tramo, un area del programa para el cual esa referencia se puede usar de manera segura.



Traits son la version de Rust de interfaces o clases base abstractas

Agregar un trait a un tipo no cuesta memoria

Genericidad y traits son relacionados.

