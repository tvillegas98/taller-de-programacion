# Clase 1 - Introducción a Rust


## ¿Qué lo hace especial a Rust?

- Previene el acceso a datos invalidos, en tiempo de compilación.
- No existen los punteros sueltos - dangling pointers, las referencias a datos inválidos son descartadas
- Previene condiciones de carrera sobre los datos al usar concurrencia.

Beneficios:

*Productividad*: Funcionalidades ergonómicas para el desarrollador, ayudas al compilar, tipos de datos sofisticados,
pattern matching, errores muy claros.


## ¿Qué compila rust?

El compilador de rust se llama C

## ¿Qué es cargo?

[Cargo]() es el gestor de paquetes de rust y el sistema de construcción (similar a make).

## Comandos utiles de C

- `cargo new` crear un nuevo proyecto
- `cargo init` crear un nuevo proyecto en un directorio existente
- `cargo build` compila el proyecto y sus dependencias
- `cargo build -release` compila el proyecto con todas las optimizaciones posibles
- `cargo run` ejecutar el proyecto
- `cargo test` ejecutar los tests
- `cargo doc` generar documentacion html
- `cargo check` analizar el proyecto, sin compilar
- `cargo fmt` Formatter de rust
- `cargo clippy` linter

---

## Tipos de datos

Tenemos tipos de datos númericos, enteros (signed/unsigned), floats.

Utilizados para indexar arrays
- usize
- isize
Dado que su espacio en memoria depende del sistema operativo, no se recomienda utilizarlos
para operaciones aritmeticas

Tambien existen otros tipos nativos como
- bool
- char (Unicode)
- Arrays
- Tuplas, no es necesario que los componentes tengan el mismo tipo
- Tipos dinamicos: Vec\<T>, String

Vec<T>, es un vector de tamaño dinámico que soporta cualquier tipo de dato.
String, son de longitud dinámica.

## Variables
Las variables son inmutables por default. Las llamamos **bindings**

```rust

let t = true;

// Para hacerlas mutables, usamos mut

let mut punto = (1_u8, 2_u8)
```

**shadow**: Se puede reutilizar el nombre de la variable, se descarta el valor anterior.

Ejemplo:

```rust

let t = true; // En este caso, definimos a t como un tipo booleano
let t = 10; // Aca pisamos el TIPO y el VALOR de la variable t, es algo que el shadow nos permite.

```


## Inferencia de Tipos

A partir del uso que les demos a la variables, el compilador podrá inferir el tipo de dato de la variable,
si el compilador decide que la lógica tiene sentido, compilará y en caso de que los tipos de datos con se condigan,
no compilará.

```rust

fn retorna_entero() -> i32{
    42 // Además, necesariamente el valor retornado sera del tipo i32.
    // De paso, no se necesita colocar el return.
}

let value = retorna_entero() // Como el compilador ya sabe que la función devuelve un i32, ya sabe que la variable
// value, es del tipo i32.

```

Otro ejemplo:

```rust

fn sumar_uno(a: i32) -> i32{
    a + 1 // No es necesario poner return, porque a + 1 es una expresión. Además el valor 1 es interpretado como
    // i32
}

```


## Estructuras

Son un conjunto de elementos puestos juntos, que son tratados como una unidad. Se le puede definir operaciones
que llamaremos métodos.

- Estructura con nombres de campos.
```rust
    struct Persona {
        nombre: String,
        apellido: String
    }
```

- Estructura tipo tupla

```rust
    struct Celsius(f32)
    struct Fahrenheit(f32)

// Permiten representar cosas como:

fn convertir_a_fahrenheit(temp: Celsius) -> Fahrenheit{
    ...
}

// Para acceder al valor de la tupla
tupla.0
tupla.N
```


- Estructura tipo unit

De tipo tupla, pero sin ningún elemento

```rust
    struct AlgunaCosa
```

## Enums

- Enumeración de items.

Solo puede tener una variante

```rust
enum Palo {
    Oro,
    Copa,
    Espada,
    Basto
}

let palo: Palo = Palo::Oro;
```

- Tagged Union, de tipo tupla

```rust
enum Carta {
    Comodin,
    Oro(u8),
    Copa(u8),
    Espada(u8),
    Basto(u8),
}

let mi_carta: Carta = Carta::Oro(9);
let mi_comodin: Carta = Carta::Comodin;

```

- De tipo estructura

```rust

enum Jugada {
    Carta {palo: Palo, numero: u8}
    Paso
}

let mi_jugada: Jugada = Jugada::Carta {palo: Palo::Copa, numero: 6}

```

## Option
Un elemento que puede contener algun valor o nada / None.
T es un generic, puede ser cualquier tipo de valor.

```rust

enum Option<T> {
    Some(T),
    None
}

fn dividir(num: f64, den: f64) -> Option<f64> {
    if den == 0.0 {
        None
    } else {
        Some(num / den)
    }
}

```

## Result
Representar el estado de error.
T es un generic, puede ser cualquier tipo de valor.
E es un generic que puede ser cualquier error.

```rust

pub enum Result<T, E> {
    Ok(T),
    Err(E)
}

fn contar_lineas_de_archivo(path: &str) -> Result<u64, String>{
    // ...
    Ok(42)
}
```


## Operaciones Utiles

- operador `?`
- `unwrap()`, `unwrap_or(42)`
- `expect("...")`
- `is_some()`, `is_none()`, `is_ok()`, `is_err()`

## Match

*match* Provee pattern matching (similar al switch de C y más debil que el de elixir), se ejecuta
la primera rama que cumple la condición

Ejemplo:
```rust
let mi_carta: Carta = Carta::Oro(9)

match mi_carta {
    Carta::Oro(valor /* Esto es un binding */) => println!("La carta es Oro de Valor {}", valor)
    _ => {}
}

```

## if let

```rust

let number = Some(42)

if let Some(i) = number {
    println!("Valor {}!", i);
}

// Solamente será ejecutado si la variable number es del tipo Some()


```

## Loop

*loop* permite indicar un loop infinito.

```rust
loop {
    ...

    if alguna_condicion {
        break;
    }
}
```

## Ciclos

Sobre una secuencia de numeros

```rust
for n in 1..101 {
    if n % 3 == 0{
        println!("{} multiplo de 3", n);
    } else if n % 4 == 0{
        println!("{} multiplo de 4 y no de 3", n);
    }else{
        println!("wea");
    }
}
```

*for* iterando sobre un iterador

```rust
let mut nombres = vec!["Juan", "Pedro"];

for nombre in nombres.iter(){
    println!("{}", nombre);
}
```

*while* realiza un bucle mientras la condicion sea true.
*while let* realiza un bucle mientras la condicion if let sea verdadera

```rust

while let Some(i) == opcional {
    // ...
}
```

## Impls

Bloques de implementacion, ***impl*** se usa para definir metodos a enums y structs.

```rust
struct Persona{
    // ...
}

impl Persona{
    fn nombre_completo(&self /* Referencia inmutable a self */) -> String{
        format!("{} {}", self.nomber, self.apellido)
    }
}

let nombre = p.nombre_completo();
```

- Funciones asociadas
En otros lenguajes, se llaman funciones estaticas

```rust
impl Persona {
    fn new(first_name: String, last_name: String) -> Persona{
        Persona{
            first_name: first_name,
            last_name: last_name,
        }
    }
}

let p = Persona::new("Pedro".to_string(), "Arce".to_string());

```

## Traits

Traits permiten definir comportamiento comun a las estructuras, similar a las interfaces en otros lenguajes.
Representan una capacidad, algo que el tipo de dato puede hacer.

```rust

trait NombreCompleto {
    fn nombre_completo(&self) -> String;
}


impl NombreCompleto for Persona {
    fn nombre_completo(&self) -> String {
        format!("{} {}", self.nombre, self.apellido);
    }
}

```

## Traits y Generics
Permiten realizar polimorfismo, aceptando tipos de datos probablmente desconocidos que implementen *Traits*

Ejemplo:

```rust

fn mostrar_nombre<W: NombreCompleto>(v: W){
    // ...
}
```

## Traits utilitarios

Existen Traits utilitarios, definidos en la std.
*Default* permite definir una construccion con valores default.

```rust

impl std::default::Default for Persona{
    fn default() -> Persona{
        nombre: "".string(),
        apellido: "".string()
    }
}

```

Se puede utilizar una implementacion default del Trait:

`#[derive(Default, Debug)]`

Donde le decimos al compilador, chequea los tipos y fijate que valor poner ahí.
Además, solo se puede derivar default, si los campos que componen el struct, también derivan de default.

## From / Into

Permiten pasar de un tipo de dato a otro.

## Modulos
Permiten organizar el código en espacios de nombre en el mismo archivo.

```rust
fn main() {
    saludos::hola()
}

mod saludos(){
    pub fn hola()
    // Todo por defecto es default.
}

```

## Crates

Un proyecto se compone por uno o mas crates.

En el archivo `Cargo.toml` agregamos las dependencias.

## Testing
- Los test son funciones de rust que verifican que el resto del codigo funciona
de la manera esperaeda

- En el cuerpo del test se hace
    - setup de los datos y estado necesario del test
    - ejecutar el codigo que se quiere testear.
    - afirmar (assert) los resultados

- Los test se organizan en un modulo test

```rust
    #[cfg(test)]
    mod tests {
        // ...
    }
```

## Repositorio de clases practicas
 - [link](https://github.com/taller-1-fiuba-rust/clases_practicas/tree/main)

- Test de integracion
    - Se colocan en el directorio tests/, al lado de src/
    - se compila cada archivo como un crate seperado. Debemos incluir como crate nuestro codigo
    - No pueden testear funciones de src/main.rs

#### Tips de la clase
- Las variables globales funcionan de manera global
- Hacer pattern matching de una expresión, es una expresión.
- None no es lo mismo que null, None sale de del problema de la existencia de null.
- Rust no tiene excepciones, a diferencia de Python o Java. No existe el burbujeo de excepciones.
- unwrap y expect unicamente son usables en test. (En la cursada)
- &str y String no son lo mismo.
- Se recomienda crear un archivo por estructura.
- Tambien que los tests Unitarios esten en el mismo archivo de la estructura
- Los tests de integracion, que esten en un archivo aparte
- cfg es una flag de compilacion, es decir, que solo compilas cosas en casos particulares
- Rust posee chequeos de exhaustividad en los match, es decir, que se deben de cubrir todos los casos o al menos
que haya un wildcard que cubra todos los otros.
