use std::fs::File;
use std::path;
use std::env;

fn main() {
    let mut argv = env::args();
    let arg: String = argv.nth(1).unwrap();

    let warp_file_path = env::home_dir().unwrap().join(path::PathBuf::from(".wrapfile"));
    let warp_file_path_string = warp_file_path.to_str().unwrap();

    match get_or_create_file(warp_file_path_string) {
        Ok(warpfile) => println!("We got the warpfile!"),
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

