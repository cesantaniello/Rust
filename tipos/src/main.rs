fn main() {

    //Tipos de datos

    // i8, i16, i32, i64, i128 -> positivos y negativos
    // u8, u16, u32, u64, u128 -> solo positivos

    let numero_uno: i8 = -10;
    let numero_dos: u8 = 10;

    //Char -> UTF-8
    let caracter = 'a';

    //Float32 o Float64
    let real: f32 = 12.5;

    //Booleano
    let resultado: bool = true;

    println!("{} {} {} {} {}", 
    numero_uno, numero_dos, caracter, real, resultado);
}
