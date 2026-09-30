mod controllers;
mod models;
mod services;

use axum::{
    Router, routing::get,
};
use dotenvy::dotenv;
use ms_general_utils::logging::logging_utils;

use crate::controllers::blocks_controller::get_block_name_controller;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    dotenv().ok();
    let _log = logging_utils::init_tracing("/tmp".to_string(), 
                                                       "pruebas.log".to_string());

    log::info!("Conexion realizada");
    let app = Router::new()
                              .route("/block/getName", get(get_block_name_controller));
    log::info!("Escuchando peticiones en 8080");
    let listener = tokio::net::TcpListener::bind("[::]:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}