use std::collections::HashMap;

use axum::Json;
use rust_decimal::Decimal;

use crate::{models::gas_models::GasSimulation, services::gas_services, utils::models::response::{Response, ResponseData}};



pub async fn simulate_gas(Json(request):Json<GasSimulation>)
                         ->Result<Json<ResponseData<HashMap<String,Decimal>>>,Json<Response>>{
    gas_services::simulate_gas(request).await.map(Json).map_err(Json)
}