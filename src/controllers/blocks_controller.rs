use axum::Json;
use serde_json::Value;

use crate::services::blocks_services::get_block_name;

//curl -X POST http://89.117.63.107:8545 -H "Content-Type: application/json" --data '{ "jsonrpc": "2.0", "method": "eth_blockNumber", "params": [], "id": 1 }'
pub async fn get_block_name_controller()->Json<Value>{
    Json(get_block_name().await)
}
