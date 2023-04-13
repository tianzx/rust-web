use std::net::TcpListener;

use rust_web::run;

#[tokio::main]
async fn main() ->std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port"); 
    // let port= listener.local_addr().unwrap().port();
    run(listener)?.await
}