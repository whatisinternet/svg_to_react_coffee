extern crate xml;
use parser::attribute::*;
use parser::util::tab_in;
use parser::util::parse_off_extra_w3c_details;
use parser::util::valid_react_dom_element;

pub fn build_element(name: xml::name::OwnedName,
                 attributes: Vec<xml::attribute::OwnedAttribute>,
                 depth: usize) -> bool{

    let temp_name: String = format!("{}", name);
    let svg_tag: String = parse_off_extra_w3c_details(temp_name);
    return print_element(svg_tag, attributes, depth);
}

    fn print_element(svg_tag: String,
                           attributes: Vec<xml::attribute::OwnedAttribute>,
                           depth: usize) -> bool {
        if valid_react_dom_element(&svg_tag) {
            print_valid_element_and_attributes(svg_tag, attributes, depth);
            return true
        } else {
            return false
        }
    }

        fn print_valid_element_and_attributes(svg_tag: String,
                           attributes: Vec<xml::attribute::OwnedAttribute>,
                           depth: usize) {
            println!("{}React.DOM.{}", tab_in(depth), svg_tag);
            build_attributes(attributes.clone(), depth);
        }
