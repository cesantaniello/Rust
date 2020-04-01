fn main() {

    let mensaje = "Hola, soy una variable en el bloque main.";
    {
        let mensaje = "Hola, soy una variable en el bloque anidado.";
        println!("Hola desde un segundo bloque.");
        println!("{}", mensaje);
        
    }
    println!("{}", mensaje);
}
