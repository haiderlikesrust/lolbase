pub mod routes;
pub mod media;
use crate::db::Database;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct State {
    pub database: Database
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub status: bool
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    pub name: String
}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameId {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueId {
    pub for_column: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RowReq {
    pub value: String,
    pub for_column: String
}