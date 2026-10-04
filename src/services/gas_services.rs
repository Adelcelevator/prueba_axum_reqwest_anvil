use std::collections::HashMap;

use rust_decimal::{Decimal, prelude::ToPrimitive};
use serde_json::{Value, to_value};

use crate::{models::gas_models::{BlockGasPriceRes, BlockGasSimulationReq, BlockGasSimulationRes, GasSimulation}, utils::{block::communication::call_blockchain, constants::blockchain_constants::BlockchainConstants, models::response::{Response, ResponseData}}};

pub async fn simulate_gas(request:GasSimulation)->Result<ResponseData<HashMap<String,Decimal>>,Response>{

    let result = request.quantity*BlockchainConstants::get_wei_value();

    let Some(value) = result.to_u128() else{
        return Err(Response{
            codigo: Some(500),
            status:None,
            mensaje:Some("Error in the convert of values.".to_string())
        });
    };

    let req = BlockGasSimulationReq{
        from:request.from,
        to: request.to,
        value:format!("0x{:x}", value),
    };

    let value = to_value(req).unwrap();
    let res = match call_blockchain::<BlockGasSimulationRes>(value, "eth_estimateGas".to_string()).await{
        Ok(simulation)=>simulation,
        Err(error)=>{
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
    };
    let hex_clean = res.result.trim_start_matches("0x");
    let gas_units: u64 = u64::from_str_radix(hex_clean, 16).unwrap_or(0);
    let total_wei = Decimal::from(gas_units) * get_gas_price().await;

    let total_eth = total_wei / BlockchainConstants::get_wei_value();

    let mut gas_simulated = HashMap::new();
    gas_simulated.insert("simulatedGasCost".to_string(),total_eth );
    Ok(ResponseData { codigo: Some(500), 
                      status: None, 
                      mensaje: Some("OK".to_string()), 
                      data: Some(gas_simulated)
                     }

    )

}

async fn get_gas_price()->Decimal{
    let res = match call_blockchain::<BlockGasPriceRes>(Value::Null, "eth_gasPrice".to_string()).await{
        Ok(simulation)=>simulation,
        Err(error)=>{
            log::error!("There was an error looking for the gas price: {}",error.error.message);
            return Decimal::from(0);
        }
    };
    let hex_clean = res.result.trim_start_matches("0x");
    let gas_price: u64 = u64::from_str_radix(hex_clean, 16).unwrap_or(0);
    Decimal::from(gas_price)
}