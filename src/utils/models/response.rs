use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response{
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub codigo: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub status: Option<String>,
    pub mensaje: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseData<T>{
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub codigo: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mensaje: Option<String>,
    pub data: Option<T>,
}