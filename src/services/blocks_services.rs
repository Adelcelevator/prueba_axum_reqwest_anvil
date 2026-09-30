use ms_general_utils::env::env_utils::get_variable;
use reqwest::Client;
use serde_json::{Value, from_str, json};

pub async fn get_block_name()->Value{
    let client = Client::new();
    let url = get_variable::<String>("DIRECCION_ANVIL").unwrap();
    let respuesta = client
                                    .post(url)
                                    .json(&json!({
                                            "jsonrpc": "2.0",
                                            "method": "eth_blockNumber",
                                            "params": [],
                                            "id": 1
                                        }))
                                    .send().await.unwrap();
    let res = respuesta.text().await.unwrap();
    from_str(&res).unwrap()
}