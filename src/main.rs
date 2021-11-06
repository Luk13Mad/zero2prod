use zero2prod::run;
use std::net:: TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener:: bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    println!("{}",format! ("http://127.0.0.1:{}", port));
    run(listener)?.await
}
