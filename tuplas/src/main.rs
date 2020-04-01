fn main() {
    let tupla = (10, false, 5.5);
    println!("El valor de la tupla 1 es: {:?}", tupla);

    let mut tupla: (i32, bool, f64, i32) = (50, true, 3.2, 64);
    println!("El valor de la tupla 2 es: {:?}", tupla);
    
    let primer_elemento = tupla.0;
    let ultimo_elemento = tupla.3;

    tupla.1 = false;

    println!("El primer elemento es: {}", primer_elemento);
    println!("El ultimo elemento es: {}", ultimo_elemento);

    println!("El valor de la tupla es: {:?}", tupla);

}
