use std::{collections::HashMap, str::FromStr};

use primitive_types::U256;

use crate::{models::{gas_models::{BlockGasSimulationReq, GasSimulation}, transaction_models::EipUnsignedTransaction}, services::{blockchain_services, gas_services::{self, esthimate_gas}, nonce_services}, utils::{block::calculations::{convert_eth_to_wei, convert_hex_to_bytes, decode_hex_to_u256}, models::response::Response}};

pub async fn execute_transaction(transaction: GasSimulation)-> Result<HashMap<String,i32>,Response>{
    let nonce = decode_hex_to_u256(nonce_services::get_nonce(&transaction.from).await?);
    let chain_id = decode_hex_to_u256(blockchain_services::get_chain_name().await?);
    let max_priority = decode_hex_to_u256(gas_services::get_max_priority_fee_per_gas().await?);
    let base_fee = decode_hex_to_u256(gas_services::get_base_fee_per_gas().await?);
    let req = BlockGasSimulationReq{
        from:transaction.from,
        to: transaction.to.clone(),
        value:format!("0x{:x}", convert_eth_to_wei(transaction.quantity)?),
    };

    let res = esthimate_gas(req).await?;

    let gas_units = decode_hex_to_u256(res);

    let transaction_unsigned = EipUnsignedTransaction{
        nonce: nonce,
        chain_id:chain_id,
        max_priority_fee_per_gas:max_priority,
        max_fee_per_gas:base_fee + max_priority,
        gas_limit:gas_units,
        to: convert_hex_to_bytes(transaction.to)?,
        value:U256::from(convert_eth_to_wei(transaction.quantity)?),
        data:vec![],
        access_list:vec![],
    };

    log::info!("{:?}",transaction_unsigned);
    Ok(HashMap::new())
}