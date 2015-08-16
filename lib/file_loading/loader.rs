use std::fs::File;
use std::io::BufReader;

pub fn get_file(file_name: String) -> BufReader<File>{
    let file = File::open(file_name).unwrap();
    return  BufReader::new(file);
}
