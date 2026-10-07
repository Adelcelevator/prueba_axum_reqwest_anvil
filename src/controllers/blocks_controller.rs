use axum::Json;
use serde_json::Value;

use crate::{services::blockchain_services::get_block_name, utils::models::response::{Response, ResponseData}};

//curl -X POST http://89.117.63.107:8545 -H "Content-Type: application/json" --data '{ "jsonrpc": "2.0", "method": "eth_blockNumber", "params": [], "id": 1 }'
pub async fn get_block_name_controller()->Result<Json<ResponseData<Value>>,Json<Response>>{
    get_block_name().await.map(Json).map_err(Json)
}
