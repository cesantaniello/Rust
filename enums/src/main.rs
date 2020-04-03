fn main() {

    enum Response {
        Success,
        Error(u32, String),
    }

    let respuesta = Response::Error(501, String::from(
        "No es posible completar la operación."
    ));

    match respuesta{
        Response::Success => println!("La petición se realizó exitosamente."),
        Response::Error(403, _) => println!("Forbidden"),
        Response::Error(404, _) => println!("Not found"),
        Response::Error(500, _) => println!("Internal server error"),
        Response::Error(_, mensaje) => println!("{}", mensaje),
    }
}
