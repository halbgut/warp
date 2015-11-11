use std::fs::File;
use std::path;
use std::env;
use std::io;
use std::io::prelude::*;

fn main() {
    let warp_file_path = env::home_dir().unwrap().join(path::PathBuf::from(".wrapfile"));
    let warp_file_path_string = warp_file_path.to_str().unwrap();

    match get_or_create_file(warp_file_path_string) {
        Ok(warpfile) => {
            match env::args().nth(1) {
                Some(ref arg) if arg == "list" => print_file(warpfile),
                Some(ref arg) => exit_proc(1, format!("Undefined action: {}", arg)),
                None => exit_proc(1, "Please pass am action.".to_string())
            }
        },
        Err(err) => println!("{:?}", err)
    }
}

fn get_or_create_file (path: &str) -> Result<File, String> {
    match File::open(path::PathBuf::from(path)) {
        Ok(warpfile) => Ok(warpfile),
        Err(err) => {
            match File::create(path::PathBuf::from(path)) {
                Ok(warpfile) => Ok(warpfile),
                Err(err) => Err(format!("Unable to create the warpfile: {:?}", err))
            }
        }
    }
}

fn exit_proc (code: i32, msg: String) {
    println!("{}", msg);
    std::process::exit(code);
}

fn print_file (mut file: File) {
    let mut warps = String::new();
    match file.read_to_string(&mut warps) {
        Ok(size) => println!("{}", warps),
        Err(err) => exit_proc(1, format!("Unable to read warpfile because: {:?}", err))
    }
}

