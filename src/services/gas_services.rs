use std::{collections::HashMap, str::FromStr};

use rust_decimal::{Decimal, prelude::Zero};
use serde_json::{Value, json, to_value};

use crate::{models::{blockchain_models::BlockChainResponse, gas_models::{BlockGasSimulationReq, BlockGasSimulationRes, GasSimulation}}, utils::{block::{calculations::{convert_eth_to_wei, decode_hex_to_u256}, communication::call_blockchain}, constants::blockchain_constants::BlockchainConstants, models::response::{Response, ResponseData}}};

pub async fn simulate_gas(request:GasSimulation)->Result<ResponseData<HashMap<String,Decimal>>,Response>{

    let req = BlockGasSimulationReq{
        from:request.from,
        to: request.to,
        value:format!("0x{:x}", convert_eth_to_wei(request.quantity)?),
    };

    let res = esthimate_gas(req).await?;

    let gas_units = Decimal::from_str(&decode_hex_to_u256(res).to_string()).map_err(|error|{
        log::error!("There was an error reading gas units: {}",error);
        return Response{
            codigo: Some(500),
            status:None,
            mensaje:Some("Error in the convert of values.".to_string())
        };
    })?;
    let total_wei = gas_units * get_gas_price().await;

    let total_eth = total_wei / BlockchainConstants::get_wei_value();

    let mut gas_simulated = HashMap::new();
    gas_simulated.insert("simulatedGasCost".to_string(),total_eth );
    Ok(ResponseData { codigo: Some(200), 
                      status: None, 
                      mensaje: Some("OK".to_string()), 
                      data: Some(gas_simulated)
                     }

    )

}

async fn get_gas_price()->Decimal{
    let res = match call_blockchain::<BlockChainResponse<String>>(Value::Null, "eth_gasPrice".to_string()).await{
        Ok(simulation)=>simulation,
        Err(error)=>{
            log::error!("There was an error looking for the gas price: {}",error.error.message);
            return Decimal::from(0);
        }
    };

    let gas_price= decode_hex_to_u256(res.result).to_string();
    Decimal::from_str(&gas_price).unwrap_or_else(|error|{
        log::error!("There was an error converting the gas price: {}",error);
        Decimal::zero()
    })
}

pub async fn esthimate_gas(req:BlockGasSimulationReq) ->Result<String,Response> {

    let params = to_value(req).map_err(|error|{
        log::error!("There is an error while converting the params for gas estimation: {}",error);
        return Response{
                codigo: Some(500),
                status:None,
                mensaje:Some("There was an error simulating the gas price.".to_string())
            };
    })?;

    call_blockchain::<BlockGasSimulationRes>(params, "eth_estimateGas".to_string()).await
    .map(|res| res.result)
    .or_else(|error|{
            log::error!("There was an error simulating the gas price: {}",error.error.message);
            if error.error.message.contains("Insufficient funds"){
                return Err(Response{
                    codigo: Some(200),
                    status:None,
                    mensaje:Some("Insufficient funds.".to_string())
                });
            }

            return Err(Response{
                codigo: Some(500),
                status:None,
                mensaje:Some("There was an error simulating the gas price.".to_string())
            });
        }
        )
}

pub async fn get_max_priority_fee_per_gas()->Result<String,Response>{
    let res = match call_blockchain::<BlockChainResponse<String>>(Value::Null, "eth_maxPriorityFeePerGas".to_string()).await{
        Ok(simulation)=>simulation,
        Err(error)=>{
            log::error!("There was an error looking for the max priority fee per gas: {}",error.error.message);
            return  Err(Response { codigo: Some(500), 
                            status: None,
                            mensaje: Some(error.error.message) 
                });
        }
    };

    Ok(res.result)
}

pub async fn get_base_fee_per_gas()->Result<String,Response>{
    let params = json!(["latest".to_string(),false]);
    let res = match call_blockchain::<BlockChainResponse<HashMap<String,Value>>>(params, "eth_getBlockByNumber".to_string()).await{
        Ok(simulation)=>simulation,
        Err(error)=>{
            log::error!("There was an error looking for the base fee per gas: {}",error.error.message);
            return  Err(Response { codigo: Some(500), 
                            status: None,
                            mensaje: Some(error.error.message) 
                });
        }
    };

    res.result.get("baseFeePerGas")
              .and_then(|v| v.as_str())
              .map(|val| val.to_string())
              .ok_or_else(||
                Response { codigo: Some(500), 
                        status: None,
                        mensaje: Some("Can't determinate base fee.".to_string()) 
                        }
    )
}