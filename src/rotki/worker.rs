mod common;
mod types;
use common::{getDirContentWithTypes, getFoldersInDir};
use types::{AssetMovements, RotkiAll};
use std::{collections::HashMap, fs::{self, ReadDir}, path::{Path, PathBuf}};


fn getTotalEurDepCoinbase(am: &Vec<Box<RotkiAll>>, m: &mut HashMap<String, f32>) -> f32{
    let mut totalEur: f32 = 0.0;
    let mut repetitions = 0;
    for i in am{
        // println!("{} next {}", &am[i].action, &am[i+1].action);
        if i.location == "binance"{
            continue;
        }
        if i.action == "buy" && i.paid_asset=="EUR"{
            match &i.paid_in_asset {
                Some(o) =>{
                    // println!("{}", o)
                        let mut key = "".to_string();
                        key.push_str(&i.action); key.push_str(&i.paid_asset); key.push_str(o); key.push_str(&i.received_asset);
                        if !m.contains_key(&key){
                            let val = o.parse::<f32>().expect("error parsing");
                            totalEur+=val;
                            m.insert(key, val);
                            continue;
                        }
                        repetitions+=1;
                }
                None => {println!("no value found")}
            }
        }

        if i.received_asset == "EUR" && i.action == "sell"{
            match &i.received_in_asset {
                Some(o) =>{
                    // println!("{}", o)
                    let mut key = "".to_string();
                    key.push_str(&i.action); key.push_str(&i.received_asset); key.push_str(o); key.push_str(&i.paid_asset);
                    totalEur-=o.parse::<f32>().expect("error parsing");
                    if !m.contains_key(&key){
                        let val = o.parse::<f32>().expect("error parsing");
                        totalEur-=val;
                        m.insert(key, val);
                        continue;
                    }
                    repetitions+=1;
                }
                None => {println!("no value found")}
            }
        }
    }
    println!("{} times same", repetitions);
    totalEur
}

pub fn findTotalEurDeposits(dir_path: &String) {
    let types = getFoldersInDir(dir_path);
    let mut m: HashMap<String, f32> = HashMap::new();
    
    println!("{:?}", types);
    let mut total = 0.0;
    for path in types{
        // let am = common::getRotkiAsset_movements(&path);
        let am = common::processRotkiAll(&path);
        // println!("{}", getTotalEurDepCoinbase(&am));
        let val = getTotalEurDepCoinbase(&am, &mut m);
        total+=val;

        println!("> {} --- {}", path, val);
    }

    println!("{:?}, {}", m, total)
    /* 
        update map with key to prevent duplicates
        calculate total deposited EUR
    */
   
}
