use serde::{Serialize, Deserialize};

use super::transaction::Transaction;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block
{
   pub block_hieght:u128,
   pub prev_hash:String,
   pub curr_hash:String,
   pub data:Vec<Transaction>,
   pub timestamp:String
}