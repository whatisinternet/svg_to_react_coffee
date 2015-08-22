
use file_loading::loader::*;
use parser::element::*;
use parser::util::*;

use xml::reader::EventReader;
use xml::reader::events::*;


pub fn parse_svg (file_name: String, header_string: String) -> Vec<String>{

    let file = get_file(file_name);
    let output_vector: Vec<String> = vec!();
    let mut parser = EventReader::new(file);

    // let mut output_vector: Vec<String> = [""];

    let mut depth = 2;
    let mut was_valid_element = false;

    for e in parser.events() {
        match e {
            XmlEvent::StartElement { name, attributes, .. } => {
                let mut element_vector: Vec<String> = build_element(name, attributes, depth);
                output_vector.push_all(element_vector.to_vec());
                was_valid_element = is_valid_element(name);
                depth += 1;
            }
            XmlEvent::EndElement { .. } => {
                depth -= 1;
            }
            XmlEvent::Characters(data) => {
                if was_valid_element {
                    depth += 1;
                    if !data.contains(">") {
                        println!("{}\"{}\"\n\n", tab_in(depth), data.to_string());
                    }
                    depth -= 1;
                }
            }
            _ => {}
        }
    }
    return output_vector;
}
