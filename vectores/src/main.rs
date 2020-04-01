fn main() {

//    let mut vector = vec![1, 2, 3];
    let mut vector: Vec<i32> = Vec::new();

    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);
    vector.push(5);

    vector.insert(0, -1);
    vector.insert(1, 0);

    vector.remove(vector.len() -1);

    vector[0] = -10;

    let primer_elemento = vector[0];
    //let ultimo_elemento = vector[vector.len() -1];
    let ultimo_elemento = vector.pop().unwrap();


    println!("El valor del vector es: {:?}", vector);

    println!("El valor del primer elemento es: {}", primer_elemento);
    println!("El valor del ultimo elemento es: {}", ultimo_elemento);


}
