fn main() {
    let mut numero = 123456789;
    let mut contador = 0;

    while numero > 0 {
        numero /= 10;
        contador += 1;
    }
    println!("La cantidad de dígitos son: {}", contador);
}
