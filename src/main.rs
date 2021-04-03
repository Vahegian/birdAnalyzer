mod common;
mod types;
use common::{getDirContentWithTypes, getFoldersInDir};
use types::{AssetMovements, RotkiAll};
use std::{fs::{self, ReadDir}, path::{Path, PathBuf}};

fn getTotalEurDepCoinbase(am: &Vec<Box<RotkiAll>>) -> f32{
    let mut totalEur: f32 = 0.0;
    for item in am{
        if item.received_asset == "ETH"{
            match &item.total_received_in_EUR {
                Some(o) =>{
                    // println!("{}", o)
                        totalEur+=o.parse::<f32>().expect("error parsing")
                }
                None => {println!("no value found")}
            }

            match &item.total_bought_cost_in_EUR {
                Some(o) =>{
                    // println!("{}", o)
                        totalEur+=o.parse::<f32>().expect("error parsing")
                }
                None => {println!("no value found")}
            }
        }
    }

    totalEur
}

fn findTotalEurDeposits(dir_path: &String) {
    let types = getFoldersInDir(dir_path);
    
    println!("{:?}", types);
    let mut total = 0.0;
    for path in types{
        // let am = common::getRotkiAsset_movements(&path);
        // println!("{:?} {}", &am, getTotalEurDepCoinbase(&am));
        let am = common::processRotkiAll(&path);
        // println!("{}", getTotalEurDepCoinbase(&am));
        total+=getTotalEurDepCoinbase(&am)
    }

    println!("{}", total)
    /* 
        update map with key to prevent duplicates
        calculate total deposited EUR
    */
   
}

fn main() {
    let rotki_data = String::from("./raw");
    findTotalEurDeposits(&rotki_data)
}
