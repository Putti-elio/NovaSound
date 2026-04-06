use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use env_logger;

use rust::database::init_database;
use rust::routes::create_router;

#[tokio::main]
async fn main() {
    env_logger::init();

    let database = init_database().expect("Failed to initialize database");

    let shared_database = Arc::new(Mutex::new(database));

    let app = create_router(shared_database);

    let address: SocketAddr = "0.0.0.0:4000"
        .parse()
        .expect("Invalid address or port");

    axum_server::bind(address)
        .serve(app.into_make_service())
        .await
        .expect("Server failed to start");
}
