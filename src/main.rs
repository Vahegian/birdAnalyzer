mod common;
mod types;
use common::{getDirContentWithTypes, getFoldersInDir};
use types::AssetMovements;
use std::{fs::{self, ReadDir}, path::{Path, PathBuf}};

fn getTotalEurDepCoinbase(am: &Vec<Box<AssetMovements>>) -> u32{
    let mut totalEur: u32 = 0;
    for item in am{
        if item.action == "deposit" && item.exchange == "coinbasepro"{
            match &item.fee_in_EUR {
                Some(o) =>{
                        totalEur+=o.parse::<u32>().expect("error parsing")
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

    for path in types{
        let am = common::getRotkiAsset_movements(&path);
        println!("{:?} {}", &am, getTotalEurDepCoinbase(&am));
    }
    /* 
        update map with key to prevent duplicates
        calculate total deposited EUR
    */
   
}

fn main() {
    let rotki_data = String::from("./raw");
    findTotalEurDeposits(&rotki_data)
}
