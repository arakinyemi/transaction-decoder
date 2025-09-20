use std::error::Error;
mod transaction;
use self::transaction::{Transaction, Decodable};


pub fn decode(transaction_hex: String) -> Result<String, Box<dyn Error>> {
    let transaction_bytes = hex::decode(transaction_hex).map_err(|e| format!("Hex Decode Error: {}", e))?;
    let mut bytes_slice = transaction_bytes.as_slice();
    let transaction = Transaction::consensus_decode(&mut bytes_slice)?;

    Ok(serde_json::to_string_pretty(&transaction)?)
}



