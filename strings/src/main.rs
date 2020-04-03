fn main() {
    //str -> Stack (Cadena inmutable)
    //String -> Heap (Cadena mutable)

    let variable_str = "Hola, soy un tipo str";

    let mut variable_string = String::from("Hola, soy un String");

    variable_string.push(',');
    variable_string.push(' ');
    variable_string.push('H');
    variable_string.push('O');
    variable_string.push('L');
    variable_string.push('A');
    variable_string.push(' ');

    variable_string.push_str("Estamos en el curso de Rust.");
    let nuevo_string = "Hola, soy un String".to_string();

    let diferente = nuevo_string != "Hola, soy una cadena".to_string();

    println!("El str es: {}", variable_str);
    println!("El String es: {}", variable_string);
    println!("El String es: {}", nuevo_string);
    println!("¿Los strings son diferentes?: {}", diferente);

}
