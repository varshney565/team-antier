use crate::model::transaction::Metadata;

use chrono::{Utc, DateTime, TimeZone};

use actix_web::{web, HttpResponse, HttpRequest};

use crate::model::transaction::Transaction;

use crate::helper::cal_hash::calculate_hash;

pub async fn index(trans : web::Json<Metadata>,req : HttpRequest) -> HttpResponse {
    let mut txn = Transaction {
        from : trans.0.from.clone(),
        to : trans.0.to.clone(),
        amount : trans.0.amount,
        txn_hash : String::from(""),
        timestamp : Utc::now().to_rfc3339(),
        status : false
    };
    let hash = calculate_hash(txn.from.clone().as_str(),txn.to.clone().as_str(),txn.amount,txn.timestamp.clone());
    txn.txn_hash = hash;
    HttpResponse::Ok().json(txn)
}