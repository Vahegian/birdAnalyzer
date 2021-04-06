use std::{env, time::{SystemTime, UNIX_EPOCH}};
extern crate base64;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use base64::{encode, decode};

#[path ="utils/yaml_parser.rs"] mod yaml_parser;
type hmac_sha256 = Hmac<Sha256>;
// #[path = "rotki/worker.rs"] mod worker;
fn main() {
    // let rotki_data = String::from("./raw");
    // worker::findTotalEurDeposits(&rotki_data)
    let args: Vec<String> = env::args().collect();
    if args.len().ne(&2) {
        println!("Please specify the path to config.yml file");
        return;
    }
    let deserialized_point = yaml_parser::parse_coinbase_pro(&args[1]);
    let secretBase64 = decode( deserialized_point.coinbase_pro_secret).expect("error decoding secret");
    let timeNow = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
    let message = format!("{}GET{}/accounts", timeNow, deserialized_point.coinbase_pro_api_url);
    let mut sign = hmac_sha256::new_varkey(&secretBase64).expect("error creating hmac key");
    sign.update(message.as_bytes());


    println!("{:?} {}", encode(&sign.finalize().into_bytes()), timeNow)
}
