use std::fs::File;
use std::fs::OpenOptions;
use std::path;
use std::env;
use std::io::prelude::*;

fn main() {
    let warp_file_path = env::home_dir().unwrap().join(path::PathBuf::from(".warpfile"));
    let warp_file_path_string = warp_file_path.to_str().unwrap();
    let warpfile = match get_or_create_file(warp_file_path_string) {
        Ok(warpfile) => warpfile,
        Err(err) => {
            println!("{:?}", err);
            return
        }
    };

    match env::args().nth(1) {
        Some(ref arg) if arg == "list" => {
            match get_file(warpfile) {
                Ok(warps) => println!("{}", warps),
                Err(err) => println!("{}", err)
            }
        },
        Some(ref arg) if arg == "add" => {
            let name = match env::args().nth(2) {
                Some(name) => name,
                None => return println!("Missing second argument for add")
            };
            let dir = match env::current_dir() {
                Ok(dir) => dir,
                Err(err) => return println!("Failed to retrieve the CWD: {:?}", err)
            };
            match add_warp_point(warpfile, name, dir) {
                Ok(_) => println!("Warp point added"),
                Err(err) => println!("{}", err)
            }
        },
        Some(ref arg) => {
            match warp(arg.to_string(), warpfile) {
                Ok(_) => (),
                Err(err) => println!("{}", err)
            }
        },
        None => println!("Please pass am action.")
    }
}

fn get_or_create_file (path: &str) -> Result<File, String> {
    match OpenOptions::new().write(true).append(true).read(true).open(path::PathBuf::from(path)) {
        Ok(warpfile) => Ok(warpfile),
        Err(_) => {
            match File::create(path::PathBuf::from(path)) {
                Ok(_) => get_or_create_file(path),
                Err(err) => Err(format!("Unable to create the warpfile: {:?}", err))
            }
        }
    }
}

fn get_file (mut file: File) -> Result<String, String> {
    let mut warps = String::new();
    match file.read_to_string(&mut warps) {
        Ok(_) => Ok(warps),
        Err(err) => Err(format!("Unable to read warpfile because: {:?}", err))
    }
}

fn add_warp_point (mut file: File, name: String, location: path::PathBuf) -> Result<(), String> {
    let location_str = match location.to_str() {
        Some(str) => str,
        None => return Err("Failed to open error".to_string())
    };
    let warp_exp = format!("{} => {}\n", name, location_str);
    match file.write_all(warp_exp.as_bytes()) {
        Err(err) => Err(format!("{:?}", err)),
        Ok(_) => Ok(())
    }
}

fn warp (target: String, warpfile: File) -> Result<(), String> {
    let warp_dir = match get_file(warpfile) {
        Ok(warps) => {
            match get_warp(warps.split_terminator('\n').collect(), target) {
                Some(warps) => warps,
                None => return Err("No warps found".to_string())
            }
        },
        Err(err) => return Err(format!("Unable to warp because: {:?}", err))
    };
    Ok(println!("{}", warp_dir))
}

/// Takes a string of warps and returns the directory the warp passed as a second argument points
/// to.
fn get_warp (mut warps: Vec<&str>, target: String) -> Option<String> {
    match warps.pop() {
        Some(pop) => {
            let vec = get_pieces(pop);
            if vec.len() == 2 && vec[0] == target {
                Some(vec[1].to_string())
            } else if vec.len() == 2 {
                get_warp(warps, target)
            } else {
                None
            }
        },
        None => None
    }
}

fn get_pieces (str: &str) -> Vec<&str> {
    str.split_terminator(" => ").collect()
}

