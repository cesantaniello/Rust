fn main() {
    //let <nombre de la variable> = <El valor>
    //let <nombre de la variable>: <Tipo> = <El valor>
    //let mut <nombre de la variable> = <El valor>

    /*
    Comentario con salto de línea
    */

    const VALOR: i32 = 10;
    let mut numero_uno = 15;
    let numero_dos: i32 = 10;

     numero_uno = 100;

    let resultado = numero_uno + numero_dos + VALOR;

    println!("EL resultado de ({} + {} + {}) es: {}",
    numero_uno, numero_dos, VALOR, resultado);
}
