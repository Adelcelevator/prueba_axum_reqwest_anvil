use serde_json::to_value;

use crate::{models::blockchain_models::BlockChainResponse, utils::{block::communication::call_blockchain, models::response::Response}};

pub async fn get_nonce(account: &str)->Result<String,Response>{
    let params = vec![String::from(account),"latest".to_string()];

    let params_value = match to_value(params){
        Ok(val)=>val,
        Err(error)=>{
            log::error!("There was an error preparing params: {}",error);
            return Err(Response{
                    codigo: Some(500),
                    status:None,
                    mensaje:Some("There was an error handling params.".to_string())
                });
        }
    };
    let res = match call_blockchain::<BlockChainResponse<String>>(params_value, "eth_getTransactionCount".to_string()).await{
        Ok(simulation)=>simulation,
        Err(error)=>{
            log::error!("There was an error looking for the gas price: {}",error.error.message);
            return Err(Response{
                    codigo: Some(500),
                    status:None,
                    mensaje:Some("There was an error in the communication.".to_string())
                });
        }
    };
    
    Ok(res.result)

}