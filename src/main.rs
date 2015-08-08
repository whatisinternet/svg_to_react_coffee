extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::EventReader;
use xml::reader::events::*;

use std::env;

fn valid_react_dom_element (test_element: &str) -> bool {
    let valid_react_elements = ["circle", "clipPath", "defs", "ellipse", "g", "line", 
        "linearGradient", "mask", "path", "pattern", "polygon", "polyline", "radialGradient",
        "rect", "stop", "svg", "text", "tspan"];
    return valid_react_elements.contains(&test_element)
}

fn parse_off_extra_w3c_details (input_string: String) -> String{
    let string_vector: Vec<&str> = input_string.split("}").collect();
    return format!("{}", string_vector[1]);
}

fn parse_svg (file_name: String){

  let full_file_path_and_name = format!("{}/{}",env::current_dir().unwrap().display(),file_name);
  println!("{}",full_file_path_and_name);
  let file = File::open(full_file_path_and_name).unwrap();
  let file = BufReader::new(file);

  let mut parser = EventReader::new(file);
  let mut depth = 0;
  for e in parser.events() {
    match e {
      XmlEvent::StartElement { name, attributes, .. } => {
        for attribute in attributes{
            let temp_name: String = format!("{}", name);
            let temp_attribute: String = format!("{}", attribute);
            let svg_tag: String = parse_off_extra_w3c_details(temp_name);
            let parsed_attribute: String = parse_off_extra_w3c_details(temp_attribute);
            println!("{} + {}      {}",  svg_tag, valid_react_dom_element(&svg_tag), parsed_attribute);
        }
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
