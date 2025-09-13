fn read_version(transaction_hex: &str) -> u32 {
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let version_bytes = &transaction_bytes[0..4];
    return 1;
}

fn main() {
    let version = read_version("")
}
