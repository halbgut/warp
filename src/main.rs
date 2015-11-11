use std::fs::File;
use std::fs::OpenOptions;
use std::path;
use std::env;
use std::io;
use std::io::prelude::*;
use std::process;

fn main() {
    let warp_file_path = env::home_dir().unwrap().join(path::PathBuf::from(".warpfile"));
    println!("{:?}", warp_file_path);
    let warp_file_path_string = warp_file_path.to_str().unwrap();

    match get_or_create_file(warp_file_path_string) {
        Ok(warpfile) => {
            match env::args().nth(1) {
                Some(ref arg) if arg == "list" => print_file(warpfile),
                Some(ref arg) if arg == "add" => {
                    let name = match env::args().nth(2) {
                        Some(name) => name,
                        None => panic!("Missing second argument for add")
                    };
                    let dir = match env::current_dir() {
                        Ok(dir) => dir,
                        Err(err) => panic!("Failed to retrieve the CWD: {:?}", err)
                    };
                    add_warp_point(warpfile, name, dir)
                },
                Some(ref arg) => panic!("Undefined action: {}", arg),
                None => panic!("Please pass am action.")
            }
        },
        Err(err) => println!("{:?}", err)
    }
}

fn get_or_create_file (path: &str) -> Result<File, String> {
    match OpenOptions::new().write(true).append(true).read(true).open(path::PathBuf::from(path)) {
        Ok(warpfile) => Ok(warpfile),
        Err(err) => {
            match File::create(path::PathBuf::from(path)) {
                Ok(warpfile) => get_or_create_file(path),
                Err(err) => Err(format!("Unable to create the warpfile: {:?}", err))
            }
        }
    }
}

fn print_file (mut file: File) {
    let mut warps = String::new();
    match file.read_to_string(&mut warps) {
        Ok(size) => println!("{}", warps),
        Err(err) => panic!("Unable to read warpfile because: {:?}", err)
    }
}

fn add_warp_point (mut file: File, name: String, location: path::PathBuf) {
    let location_str = match location.to_str() {
        Some(str) => str,
        None => {
            panic!("Failed to open error");
            return
        }
    };
    let warp_exp = format!("\n{} => {}", name, location_str);
    file.write_all(warp_exp.as_bytes());
    println!("Warp point added\n{}\n", warp_exp);
}

