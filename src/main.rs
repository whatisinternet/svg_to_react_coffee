extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::EventReader;
use xml::reader::events::*;

use std::env;

fn parse_svg (file_name: String){

  let full_file_path_and_name = format!("{}/{}",env::current_dir().unwrap().display(),file_name);
  println!("{}",full_file_path_and_name);
  let file = File::open(full_file_path_and_name).unwrap();
  let file = BufReader::new(file);

  let mut parser = EventReader::new(file);
  let mut depth = 0;
  for e in parser.events() {
    match e {
      XmlEvent::StartElement { name, .. } => {
        println!("{}+{}+{}", depth, name,);
      }
      _ => {}
    }
  }
}


fn main() {
  let svg_file: Vec<_> = env::args().collect();
  let file_name: Vec<&str> = svg_file[1].split(".svg").collect();
  if file_name.len() > 1 {
    parse_svg(svg_file[1].clone());
  }
}
