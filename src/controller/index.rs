use crate::model::transaction::{Metadata, Status};

use chrono::{Utc, DateTime, TimeZone};

use actix_web::{web, HttpResponse, HttpRequest};
use rocksdb::{Options, DB};

use crate::model::transaction::Transaction;

use crate::helper::cal_hash::calculate_hash;

pub async fn index(trans : web::Json<Metadata>,req : HttpRequest) -> HttpResponse {
    let mut txn = Transaction {
        from : trans.0.from.clone(),
        to : trans.0.to.clone(),
        amount : trans.0.amount,
        txn_hash : String::from(""),
        timestamp : Utc::now().to_rfc3339(),
        status : Status::PENDING
    };
    let hash = calculate_hash(txn.from.clone().as_str(),txn.to.clone().as_str(),txn.amount,txn.timestamp.clone());
    txn.txn_hash = hash;
    let path = "./Databases/Transactions";
    let mut opts = Options::default();
    opts.create_if_missing(true);
    let db = DB::open(&opts, path).expect("Failed to open database Transaction");
    let transaction_serialized =
        serde_json::to_string(&txn).expect("Failed to serialize transaction");
    db.put(
        &txn.txn_hash.as_bytes(),
        &transaction_serialized,
    )
    .expect("Failed to store trasaction");

    HttpResponse::Ok().json(txn)
}