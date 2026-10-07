mod controllers;
mod models;
mod services;
mod utils;

use axum::{
    Router, routing::{get, post},
};
use dotenvy::dotenv;

use crate::{controllers::{blocks_controller::get_block_name_controller, transaction_controller::{execute_transaction, simulate_gas}}, utils::logging::logging_utils};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    dotenv().ok();
    let _log = logging_utils::init_tracing("/tmp".to_string(), 
                                                       "pruebas.log".to_string());

    let block_routes = Router::new()
                                .route("/getName", get(get_block_name_controller));
    
    let transaction_routes = Router::new()
                                .route("/simulateGas", post(simulate_gas))
                                .route("/executeTransaction", post(execute_transaction));

    let app = Router::new().nest("/blockTest", Router::new()
                                                                            .nest("/block", block_routes)
                                                                            .nest("/transaction", transaction_routes));
    log::info!("Escuchando peticiones en 8080");
    let listener = tokio::net::TcpListener::bind("[::]:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}