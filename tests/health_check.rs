#[tokio::test] 
async fn health_check_works() {
    // Arrange 
    spawn_app();
}

fn spawn_app() { 
    let server = rust_web::run().expect("failed to bind address");
    let _ = tokio::spawn(server);
}