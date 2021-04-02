mod common;
mod types;
use common::{getDirContentWithTypes, getFoldersInDir};
use std::{
    fs::{self, ReadDir},
    path::{Path, PathBuf},
};
use types::{DirType, RotkiAll};

fn processRotkiAll() {
    let csvf = fs::read_to_string("./raw/all_events.csv");
    let csvu = csvf.unwrap();
    let mut reader = csv::Reader::from_reader(csvu.as_bytes());
    let mut data: Vec<Box<RotkiAll>> = vec![];
    for record in reader.deserialize() {
        // let mut record: RotkiAll = record.unwrap();
        data.push(Box::new(record.unwrap()));
        // println!("{:?}", record);
    }

    // let d = data[0];

    println!("{:?}", data[0].net_profit_or_loss.as_ref().unwrap());
}

fn findTotalEurDeposits(dir_path: &String) {
    // for item in dirs{
    //     if
    // }
    // let types = getDirContentWithTypes(dir_path);
    let types = getFoldersInDir(dir_path);
    println!("{:?}", types)
}

fn main() {
    let rotki_data = String::from("./raw");
    findTotalEurDeposits(&rotki_data)
}
