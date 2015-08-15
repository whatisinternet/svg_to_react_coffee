
use file_loading::loader::*;
use parser::element::*;
use parser::util::*;

use xml::reader::EventReader;
use xml::reader::events::*;


pub fn parse_svg (file_name: String){

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
