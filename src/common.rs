use std::{fs::{self, ReadDir}, process};

use crate::types::DirType;

pub fn getDirContentWithTypes(dir_path: &String) -> Vec<DirType> {
    let dirs = fs::read_dir(dir_path);
    let dirs = match dirs {
        Ok(content) => content,
        Err(error) => {
            println!("Error at the path {}, {:?}", dir_path, error) ;
            process::exit(0)
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

pub fn getFoldersInDir(dir_path: &String) -> Vec<String>{
    let file_types = getDirContentWithTypes(dir_path);
    let mut dirs:Vec<String> = vec![];
    for file in file_types{
        if file.isDir{
            let mut path = String::from("");
            path.push_str(dir_path);
            path.push_str("/"); path.push_str(&file.name);
            dirs.push(path);
        }
    }
    dirs
}
