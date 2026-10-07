use primitive_types::U256;
use rust_decimal::{Decimal, prelude::ToPrimitive};

use crate::utils::{constants::blockchain_constants::BlockchainConstants, models::response::Response};

pub fn decode_hex_to_u256(coded:String)->U256{
    let hex_clean = coded.trim_start_matches("0x");
    U256::from_str_radix(hex_clean, 16).unwrap_or(U256::zero())
}

pub fn convert_eth_to_wei(quantity:Decimal)->Result<u128, Response>{
     let result = quantity*BlockchainConstants::get_wei_value();

    result.to_u128().ok_or_else(||{
        return Response{
            codigo: Some(500),
            status:None,
            mensaje:Some("Error in the convert of values.".to_string())
        };
    })
}

pub fn convert_hex_to_bytes(direction:String)->Result<Vec<u8>,Response>{
    let address = direction.strip_prefix("0x").ok_or_else(||{
        return Response{
            codigo: Some(500),
            status:None,
            mensaje:Some("Error trying to convert values.".to_string())
        };
    })?;

    let bytes:Result<Vec<u8>, Response> = (0..address.len())
    .step_by(2)
    .map(|i| {
        let val  = u8::from_str_radix(&address[i..i + 2], 16).map_err(|error|{
        log::error!("There was an error converting the direction to bytes: {}",error);
        return  Response{
            codigo: Some(500),
            status:None,
            mensaje:Some("Error trying to convert values.".to_string())
        };
    })?;
    Ok(val)
    })
    .collect();

    Ok(bytes?)
}