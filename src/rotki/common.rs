use std::fs;

use crate::worker::types::{AssetMovements, DirType, RotkiAll};

pub fn getDirContentWithTypes(dir_path: &String) -> Vec<DirType> {
    let dirs = fs::read_dir(dir_path);
    let dirs = match dirs {
        Ok(content) => content,
        Err(error) => {
            println!("Error at the path {}, {:?}", dir_path, error);
            return vec![];
            // process::exit(0)
        }
    };
    let names = dirs
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path().file_name().and_then(|n| {
                    n.to_str().map(|s| {
                        let mut full_path = String::from("");
                        full_path.push_str(dir_path);
                        full_path.push_str("/");
                        full_path.push_str(s);
                        let meta = fs::metadata(full_path);
                        DirType {
                            name: String::from(s),
                            isDir: meta.unwrap().file_type().is_dir(),
                        }
                    })
                })
            })
        })
        .collect::<Vec<DirType>>();
    names
}

pub fn getFoldersInDir(dir_path: &String) -> Vec<String> {
    let file_types = getDirContentWithTypes(dir_path);
    let mut dirs: Vec<String> = vec![];
    for file in file_types {
        if file.isDir {
            let mut path = String::from("");
            path.push_str(dir_path);
            path.push_str("/");
            path.push_str(&file.name);
            dirs.push(path);
        }
    }
    dirs
}

pub fn processRotkiAll(dir_path: &String) -> Vec<Box<RotkiAll>> {
    let mut full_path = String::from("");
    full_path.push_str(dir_path);
    full_path.push_str("/all_events.csv");
    let csvf = fs::read_to_string(full_path);
    let csvu = csvf.unwrap();
    let mut reader = csv::Reader::from_reader(csvu.as_bytes());
    let mut data: Vec<Box<RotkiAll>> = vec![];
    for record in reader.deserialize() {
        data.push(Box::new(record.unwrap()));

    }
    
    data
}

pub fn getRotkiAsset_movements(dir_path: &String) -> Vec<Box<AssetMovements>> {
    let mut full_path = String::from("");
    full_path.push_str(dir_path);
    full_path.push_str("/asset_movements.csv");
    let csvf = fs::read_to_string(full_path);
    let csvu = csvf.unwrap();
    let mut reader = csv::Reader::from_reader(csvu.as_bytes());
    let mut data: Vec<Box<AssetMovements>> = vec![];
    for record in reader.deserialize() {
        data.push(Box::new(record.unwrap()));

    }

    data
}
