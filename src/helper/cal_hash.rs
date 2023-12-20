use sha2::{Digest, Sha256};
use hex;

pub fn calculate_hash(from: &str, to: &str, amount: f64, timestamp: String) -> String {
    let data = format!("{}{}{}{}", from, to, amount, timestamp);

    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    hex::encode(result)
}