extern crate svg_to_react;

use svg_to_react::convert;
use std::env;

fn main() {
    let svg_file: Vec<_> = env::args().collect();
    let file_name: String = svg_file[1].to_string();
    let full_file_path_and_name = format!("{}/{}",env::current_dir().unwrap().display(),file_name);
    convert(full_file_path_and_name.clone());
}
