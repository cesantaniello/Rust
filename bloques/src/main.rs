fn main() {
    /*
    let mensaje = "Hola, soy una variable en el bloque main.";
    {
        let mensaje = "Hola, soy una variable en el bloque anidado.";
        println!("Hola desde un segundo bloque.");
        println!("{}", mensaje);
        
    }
    println!("{}", mensaje);
    */
    let calificacion: i8 = 10;

    let mensaje = if calificacion == 10 {
        String::from("¡Felicitaciones!")
    }else {
        String::from("Necesitas estudiar más")
    };
    println!("{}", mensaje);

}
