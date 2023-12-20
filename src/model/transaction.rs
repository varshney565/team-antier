use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Transaction {
    pub from : String,
    pub to : String,
    pub amount : f64,
    pub txn_hash : String,
    pub timestamp : String,
    pub status : Status
}

#[derive(Serialize,Deserialize,Debug)]
pub struct Metadata {
    pub from : String,
    pub to : String,
    pub amount : f64
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    PENDING,
    PROCESSING,
    ACCEPTED,
    REJECTED
}
