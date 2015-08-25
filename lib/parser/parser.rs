
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
                let temp_name: String = format!("{}", name.clone());
                let mut extracted_attributes: Vec<String> = vec!();
                for attribute in attributes {
                    extracted_attributes.push(format!("{}", attribute));
                }
                for start_element_and_attribute in parse_start_element(temp_name.clone(),
                                                                       extracted_attributes,
                                                                       depth){
                    output_vector.push(start_element_and_attribute)
                }
                was_valid_element = is_valid_element(temp_name.clone());
                depth += 1;
            }
            XmlEvent::EndElement { .. } => {
                depth -= 1;
            }
            XmlEvent::Characters(data) => {
                if was_valid_element {
                    depth += 1;
                    output_vector.push(parse_character(data, depth));
                    depth -= 1;
                }
            }
            _ => {}
        }
    }
    return output_vector;
}
    fn parse_start_element( name: String,
                            attributes: Vec<String>,
                            depth: usize ) -> Vec<String> {

        let mut output_vector: Vec<String> = vec!();
        let element_vector: Vec<String> =
            build_element(name.clone(), attributes, depth);

        for completed_element in element_vector {
            output_vector.push(completed_element.clone());
        }
        return output_vector;
    }

    fn parse_character(data: String ,depth: usize) -> String {
        if !data.contains(">") {
            return format!("{}\"{}\"\n\n", tab_in(depth), data.to_string());
        }
        return "".to_string();
    }
