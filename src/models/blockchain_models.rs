use serde::{Deserialize, Serialize};


#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct BlockChainErrorRequest{
    pub jsonrpc:String,
    pub id: i32,
    pub error:BlockChainError,
}

#[derive(Clone,Serialize,Deserialize,Debug)]
pub struct BlockChainError{
    pub code:i32,
    pub message:String,
}

#[derive(Clone, Serialize, Deserialize,Debug)]
pub struct BlockChainResponse<T>{
    pub id: i32,
    pub jsonrpc: String,
    pub result: T,
}