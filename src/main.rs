use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferTx {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub nonce: u64,
    pub signature: String,
}
fn main() {
    let json = r#" {
  "from": "0x8f3a9cD4eB2F6a91C7dD4b1E2F9aA6c3B8d1eF45",
  "to": "0xA12b9F7cD9e3B5a0F8c6D1e4A9b7F3c2d8E6f1A90",
  "amount": 1000,
  "nonce": 1,
  "signature": "base64_or_hex_signature"
}
  "#;
    let parsed: TransferTx = read_json(json);

    println!(
        "from {} to addres {}, {} amount has been transferd",
        parsed.from, parsed.to, parsed.amount
    );
}

fn read_json(raw_json: &str) -> TransferTx {
    let transfer_tx: TransferTx = serde_json::from_str(raw_json).unwrap();
    transfer_tx
}
