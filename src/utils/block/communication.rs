use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, from_str, from_value, json};

use crate::{models::blockchain_models::{BlockChainError, BlockChainErrorRequest}, utils::env::env_utils::get_variable};


pub async fn call_blockchain<R:Serialize+DeserializeOwned>(params:Value, method:String)->Result<R,BlockChainErrorRequest>{
    let client = Client::new();
    let Some(url) = get_variable::<String>("DIRECCION_ANVIL")else{
        let error = BlockChainError{
            code:0,
            message:"Can't determinate the blockchain url".to_string()
        };
        return Err(BlockChainErrorRequest { jsonrpc: "".to_string(), id: 0, error: error });
    };
    
    let par  = match params {
        Value::Null =>vec![],
        Value::Array(arr) => arr,
        _ =>vec![ params],
    };
    let respuesta = match client
                                    .post(url)
                                    .json(&json!({
                                            "jsonrpc": "2.0",
                                            "method": method,
                                            "params": par,
                                            "id": 1
                                        }))
                                    .send().await {
        Ok(res)=>res,
        Err(error)=>{
            log::error!("There is an error in the communication to the blockchain: {}",error);
            let error = BlockChainError{
                code:0,
                message:"Can't communicate to the blockchain.".to_string()
            };
            return Err(BlockChainErrorRequest { jsonrpc: "2.0".to_string(), id: 0, error: error });
        }
    };

    let texto = match respuesta.text().await {
        Ok(res)=>res,
        Err(error)=>{
            log::error!("There is an error in the conversion the blockchain response: {}",error);
            let error = BlockChainError{
                code:0,
                message:"Can convert to the blockchain response.".to_string()
            };
            return Err(BlockChainErrorRequest { jsonrpc: "".to_string(), id: 0, error: error });
        }
    };
    
    let value:Value = match from_str(&texto) {
        Ok(res)=>res,
        Err(error)=>{
            log::error!("There is an error in the conversion the blockchain response to value: {}",error);
            let error = BlockChainError{
                code:0,
                message:"Can convert the blockchain response to value.".to_string()
            };
            return Err(BlockChainErrorRequest { jsonrpc: "".to_string(), id: 0, error: error });
        }
    };

    if value.get("error").is_none() {
        return match from_value::<R>(value) {
            Ok(res)=>Ok(res),
            Err(error)=>{
                log::error!("There is an error in the conversion of the result: {}",error);
                let error = BlockChainError{
                    code:0,
                    message:"Can't convert the result.".to_string()
                };
                return Err(BlockChainErrorRequest { jsonrpc: "".to_string(), id: 0, error: error });
            }
        };
    }

    match from_value::<BlockChainErrorRequest>(value) {
            Ok(res)=>Err(res),
            Err(error)=>{
                log::error!("There is an error in the response from the blockchain: {}",error);
                let error = BlockChainError{
                    code:0,
                    message:"Can convert the blockchain response in legible data.".to_string()
                };
                return Err(BlockChainErrorRequest { jsonrpc: "".to_string(), id: 0, error: error });
            }
        }

}