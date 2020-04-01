fn main() {
    let numeros = [1, 2, 3, 4, 5];
    println!("El valor del arreglo 1 es: {:?}", numeros);

    let mut numeros: [i32; 6] = [6, 7, 8, 9, 10, 11];
    println!("El valor del arreglo 2 es: {:?}", numeros);
    
    let valores = [1; 5];
    println!("El valor del arreglo es: {:?}", numeros);

    let primer_elemento = numeros[0];
    let ultimo_elemento = numeros[numeros.len() -1];

    numeros[2] = 30;

    println!("El elemento es: {}", primer_elemento);
    println!("El elemento es: {}", ultimo_elemento);

    println!("El valor del arreglo es: {:?}", numeros);

}
