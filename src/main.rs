extern crate xml;
extern crate case;

use std::fs::File;
use std::io::BufReader;

use xml::reader::EventReader;
use xml::reader::events::*;

use case::CaseExt;

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

fn tab_in(depth: usize) -> String {
    let mut tabs = "".to_string();
    for i in (1..).take(depth){
        tabs = format!("{}  ",&tabs);
    }
    return format!("{}",&tabs);
}

fn transform_attribute (attribute: String) -> String {
    let attribute_key_value: Vec<&str> = attribute.split("=").collect();
    let attribute_key: &str = attribute_key_value[0];
    let attribute_value: &str = attribute_key_value[1];
    return formatted_attribute(attribute_key, attribute_value.clone());
}

fn formatted_attribute ( attribute_key: &str, attribute_value: &str) -> String {
    let mut attributes = "".to_string();
    if attribute_key == "style" {
        attributes = format!("{}: {{ {}\" }}",
                             attribute_key
                             .replace("-","_")
                             .to_camel_lowercase(),
                             attribute_value
                                .replace(",","")
                                .replace("-","_")
                                .replace("\"","")
                                .replace(":",": \"")
                                .replace(";","\", ")
                                .to_string()
                                .to_camel_lowercase());
    } else {
        if attribute_key.contains(":"){
            attributes = format!("\"{}\": {}",
                                 attribute_key
                                 .to_string()
                                 .replace("-", "_")
                                 .to_camel_lowercase(),
                                 attribute_value);
        }
        else{
            attributes = format!("{}: {}",
                                 attribute_key
                                 .replace("-","_")
                                 .to_camel_lowercase(),
                                 attribute_value);
        }
    }
    return attributes.to_string();
}

fn build_attributes(attributes: Vec<xml::attribute::OwnedAttribute>, depth: usize) {
    if attributes.len() == 1 {
        for attribute in attributes{
            let temp_attribute: String = format!("{}", attribute);
            let parsed_attribute: String = parse_off_extra_w3c_details(temp_attribute);
            println!("{}{}", tab_in(depth + 1), transform_attribute(parsed_attribute));
        }
    }else {
        for attribute in attributes{
            let temp_attribute: String = format!("{}", attribute);
            let parsed_attribute: String = parse_off_extra_w3c_details(temp_attribute);
            println!("{}{}", tab_in(depth + 1), transform_attribute(parsed_attribute));
        }
    }
    println!("{},", tab_in(depth +1));
}

fn build_element(name: xml::name::OwnedName,
                 attributes: Vec<xml::attribute::OwnedAttribute>,
                 depth: usize) {

    let temp_name: String = format!("{}", name);
    let svg_tag: String = parse_off_extra_w3c_details(temp_name);
    if valid_react_dom_element(&svg_tag) {
        println!("{}React.DOM.{}", tab_in(depth), svg_tag);
        build_attributes(attributes.clone(), depth);
    }
}

fn get_file(file_name: String) -> BufReader<std::fs::File>{
    let full_file_path_and_name = format!("{}/{}",env::current_dir().unwrap().display(),file_name);
    let file = File::open(full_file_path_and_name).unwrap();
    return  BufReader::new(file);
}


fn parse_svg (file_name: String){

    let file = get_file(file_name);
    let mut parser = EventReader::new(file);

    let mut depth = 2;
    for e in parser.events() {
        match e {
            XmlEvent::StartElement { name, attributes, .. } => {
                build_element(name, attributes, depth);
                depth += 1;
            }
            XmlEvent::EndElement { .. } => {
                depth -= 1;
            }
            XmlEvent::Characters(data) => {
                depth += 1;
                if !data.contains(">") {
                    println!("{}\"{}\"\n\n", tab_in(depth), data.to_string());
                }
                depth -= 1;
            }
            _ => {}
        }
    }
}


fn main() {
  let svg_file: Vec<_> = env::args().collect();
  let file_name: Vec<&str> = svg_file[1].split(".svg").collect();
  println!("module.exports = React.createFactory React.createClass\n\n  render: ->");
  if file_name.len() > 1 {
    parse_svg(svg_file[1].clone());
  }
}
