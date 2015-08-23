
use file_loading::loader::*;
use parser::element::*;
use parser::util::*;

use xml::reader::EventReader;
use xml::reader::events::*;


pub fn parse_svg (file_name: String, header_string: String) -> Vec<String>{

    let file = get_file(file_name);
    let mut output_vector: Vec<String> = vec!(header_string);
    let mut parser = EventReader::new(file);

    // let mut output_vector: Vec<String> = [""];

    let mut depth = 2;
    let mut was_valid_element = false;

    for e in parser.events() {
        match e {
            XmlEvent::StartElement { name, attributes, .. } => {
                let element_vector: Vec<String> = build_element(name.clone(), attributes, depth);
                for completed_element in element_vector {
                    output_vector.push(completed_element.clone());
                }
                was_valid_element = is_valid_element(name.clone());
                depth += 1;
            }
            XmlEvent::EndElement { .. } => {
                depth -= 1;
            }
            XmlEvent::Characters(data) => {
                if was_valid_element {
                    depth += 1;
                    if !data.contains(">") {
                        let valid_data: String = format!("{}\"{}\"\n\n", tab_in(depth), data.to_string());
                        output_vector.push(valid_data);

                    }
                    depth -= 1;
                }
            }
            _ => {}
        }
    }
    return output_vector;
}
