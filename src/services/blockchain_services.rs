use serde_json::Value;

use crate::{models::blockchain_models::BlockChainResponse, utils::{block::communication::call_blockchain, models::response::{Response, ResponseData}}};

pub async fn get_block_name()->Result<ResponseData<Value>,Response>{
    match call_blockchain::<Value>(Value::Null, "eth_blockNumber".to_string()).await{
        Ok(res)=>{
            Ok(ResponseData{
                codigo: Some(200),
                status:None,
                mensaje: Some("ok".to_string()),
                data: Some(res)
            })
        },
        Err(error)=>{
            Err(Response { codigo: Some(500), 
                            status: None,
                            mensaje: Some(error.error.message) 
                })
        }
    }
}

pub async fn get_chain_name()->Result<String,Response>{
    match call_blockchain::<BlockChainResponse<String>>(Value::Null, "eth_chainId".to_string()).await{
        Ok(res)=>{
            Ok(res.result)
        },
        Err(error)=>{
            Err(Response { codigo: Some(500), 
                            status: None,
                            mensaje: Some(error.error.message) 
                })
        }
    }
}