use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RotkiAll{
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
    total_received_in_EUR: Option<String>
}

fn main() {
    let csvf = fs::read_to_string("./raw/all_events.csv");
    let csvu = csvf.unwrap();
    let mut reader = csv::Reader::from_reader(csvu.as_bytes());
    for record in reader.deserialize(){
        let record: RotkiAll = record.unwrap();
        println!(
            "{:?}",
            record
        );
    }
}
