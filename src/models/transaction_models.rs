use primitive_types::U256;

#[derive(Clone,Debug)]
pub struct EipUnsignedTransaction {
    pub chain_id: U256,
    pub nonce: U256,
    pub max_priority_fee_per_gas: U256,
    pub max_fee_per_gas: U256,
    pub gas_limit: U256,
    pub to: Vec<u8>,
    pub value: U256,
    pub data: Vec<u8>,
    pub access_list:Vec<u8>,
}