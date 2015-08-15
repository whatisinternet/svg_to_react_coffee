use std::fs::File;
use std::io::BufReader;
use std::env;

pub fn get_file(file_name: String) -> BufReader<File>{
    let full_file_path_and_name = format!("{}/{}",env::current_dir().unwrap().display(),file_name);
    let file = File::open(full_file_path_and_name).unwrap();
    return  BufReader::new(file);
}
