fn saludar_usuarios() {
    println!("Hola, desde una función");
}

fn suma(numero_uno: i32, numero_dos: i32) -> i32 {
    numero_uno + numero_dos
}

fn factorial(numero: u32) -> u32 {
    if numero == 1 {
        numero
    } else {
        factorial(numero - 1) * numero
    }
}

fn main() {
    saludar_usuarios();

    let resultado_suma = suma(10, 20);
    let resultado_factorial = factorial(5);

    println!("El resultado de la suma es: {}", resultado_suma);
    println!("El resultado del factorial es: {}", resultado_factorial);

}
