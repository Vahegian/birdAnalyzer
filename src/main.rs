use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct RotkiAll {
    #[serde(rename = "type")]
    action: String,
    location: String,
    paid_asset: String,
    paid_in_asset: Option<String>,
    taxable_amount: Option<String>,
    received_asset: Option<String>,
    received_in_asset: Option<String>,
    net_profit_or_loss: Option<String>,
    time: Option<String>,
    is_virtual: Option<String>,
    paid_in_EUR: Option<String>,
    taxable_received_in_EUR: Option<String>,
    taxable_bought_cost_in_EUR: Option<String>,
    cost_basis: Option<String>,
    total_bought_cost_in_EUR: Option<String>,
    total_received_in_EUR: Option<String>,
}

fn main() {
    let csvf = fs::read_to_string("./raw/all_events.csv");
    let csvu = csvf.unwrap();
    let mut reader = csv::Reader::from_reader(csvu.as_bytes());
    let mut data: Vec<Box<RotkiAll>> = vec!();
    for record in reader.deserialize() {
        // let mut record: RotkiAll = record.unwrap();
        data.push(Box::new(record.unwrap()));
        // println!("{:?}", record);
    }

    // let d = data[0];

    println!("{:?}", data[0].net_profit_or_loss.as_ref().unwrap());
}
