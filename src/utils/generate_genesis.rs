use chrono::{Local, Utc};
use ring::digest::{Context, SHA256};
use rocksdb::{Options, DB};

use crate::model::block::Block;

use super::block_hash::block_hash;

pub fn generate_genesis() {
    let genesis_block = Block {
        block_hieght:1,
        prev_hash: "".to_string(),
        curr_hash: block_hash(Vec::new(),"".to_string(),Utc::now().to_string()),
        data: Vec::new(),
        timestamp: Utc::now().to_string(),
    };
    let path = "./Databases/Blockchain";
    let mut opts = Options::default();
    opts.create_if_missing(true);
    let db = DB::open(&opts, path).expect("Failed to open database");
    let genesis_block_serialized =
        serde_json::to_string(&genesis_block).expect("Failed to serialize");
    db.put(
        &genesis_block.block_hieght.to_string().as_bytes(),
        &genesis_block_serialized,
    )
    .expect("Failed to store genesis block");
}
