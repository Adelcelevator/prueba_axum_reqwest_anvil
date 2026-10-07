use std::collections::HashMap;

use axum::Json;
use rust_decimal::Decimal;

use crate::{models::gas_models::GasSimulation, services::{gas_services, transaction_services}, utils::models::response::{Response, ResponseData}};



pub async fn simulate_gas(Json(request):Json<GasSimulation>)
                         ->Result<Json<ResponseData<HashMap<String,Decimal>>>,Json<Response>>{
    gas_services::simulate_gas(request).await.map(Json).map_err(Json)
}

pub async fn execute_transaction(Json(request):Json<GasSimulation>)
                         ->Result<Json<HashMap<String,i32>>,Json<Response>>{
    transaction_services::execute_transaction(request).await.map(Json).map_err(Json)
}