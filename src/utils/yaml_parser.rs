use serde::{Deserialize};
use std::{env, io::Read};

#[derive(Debug, Deserialize)]
pub struct CoinBaseConfig {
    pub coinbase_pro_api: String,
    pub coinbase_pro_secret: String,
    pub coinbase_pro_passphrase: String,
    pub coinbase_pro_api_url: String,
}

fn getFileContent(path: &String)->String{
    let mut f = std::fs::File::open(path).expect("error reading the yaml file");
    let mut content: String = String::new();
    f.read_to_string(&mut content).expect("Unable to read yml data"); 
    content
}

pub fn parse_coinbase_pro(path: &String) -> CoinBaseConfig{
    let content = getFileContent(path);
    let deserialized_point: CoinBaseConfig = serde_yaml::from_str(&content).expect("error parsing yaml");
    // println!("{:?}", deserialized_point)
    deserialized_point 
}
