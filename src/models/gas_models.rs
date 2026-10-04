use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct GasSimulation{
    pub from: String,
    pub to: String,
    pub quantity: Decimal,
}

#[derive(Clone, Serialize, Deserialize,Debug)]
pub struct BlockGasSimulationReq{
    pub from: String,
    pub to: String,
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize,Debug)]
pub struct BlockGasSimulationRes{
    pub id: i32,
    pub jsonrpc: String,
    pub result: String,
}

#[derive(Clone, Serialize, Deserialize,Debug)]
pub struct BlockGasPriceRes{
    pub id: i32,
    pub jsonrpc: String,
    pub result: String,
}