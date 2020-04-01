use std::io;

fn main() {
    //Nombre
    println!("Ingresa el nombre de usuario: ");
    let mut username = String::new(); //Static -> ""
    //Result -> Success or Error
    io::stdin().read_line(&mut username);    //Referencia
    let username = username.trim();
    
    //Edad
    println!("Ingresa la edad de usuario: ");
    let mut edad = String::new(); 
    io::stdin().read_line(&mut edad);
    let edad = edad.trim();
    //Result
    let edad: i32 = edad.parse().unwrap();
    
    //Resultado final en pantalla
    println!("Hola, {}. Tu edad es: {}", username, edad);
}
