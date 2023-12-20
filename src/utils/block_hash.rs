use hex::encode;
use ring::digest::{Context, SHA256};
use sha2::Sha256;

use crate::model::{block::Block, transaction::Transaction};

pub fn block_hash(txn:Vec<Transaction>,prev_hash:String,timestamp:String) -> String {
    let mut hash = Context::new(&SHA256);
    hash.update(timestamp.as_bytes());
    let transactions_json = serde_json::to_string(&txn).expect("Error while serializing");
    hash.update(transactions_json.as_bytes());
    hash.update(prev_hash.as_bytes());
    let val = encode(hash.finish());
    return val;
}
