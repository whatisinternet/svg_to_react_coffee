extern crate xml;
use parser::attribute::*;
use parser::util::tab_in;
use parser::util::parse_off_extra_w3c_details;
use parser::util::valid_react_dom_element;

pub fn is_valid_element(name: xml::name::OwnedName) -> bool {
    let temp_name: String = format!("{}", name);
    let svg_tag: String = parse_off_extra_w3c_details(temp_name);
    if valid_react_dom_element(&svg_tag) {
        return true
    }
    return false
}


pub fn build_element(name: xml::name::OwnedName,
                 attributes: Vec<xml::attribute::OwnedAttribute>,
                 depth: usize) -> Vec<String>{

    let temp_name: String = format!("{}", name);
    let svg_tag: String = parse_off_extra_w3c_details(temp_name);
    let element_with_attributes: Vec<String> = element(svg_tag, attributes, depth);
    return element_with_attributes.clone();
}

    fn element(svg_tag: String,
                           attributes: Vec<xml::attribute::OwnedAttribute>,
                           depth: usize) -> Vec<String>{
        let mut valid_element_and_attributes: Vec<String> = vec!("".to_string());
        if valid_react_dom_element(&svg_tag) {
            valid_element_and_attributes = element_and_attributes(svg_tag, attributes, depth);
        }
        return valid_element_and_attributes;
    }

        fn element_and_attributes(svg_tag: String,
                           attributes: Vec<xml::attribute::OwnedAttribute>,
                           depth: usize) -> Vec<String>{

            let mut dom_element: Vec<String> = vec!(format!("{}React.DOM.{}", tab_in(depth), svg_tag));

            let dom_element_attributes: Vec<String> = build_attributes(attributes.clone(), depth);
            for attribute in dom_element_attributes {
                dom_element.push(attribute);
            }

            return dom_element;
        }
