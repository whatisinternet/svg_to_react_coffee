
extern crate xml;
use parser::attribute::*;
use parser::util::*;

fn valid_react_dom_element (test_element: &str) -> bool {
    let valid_react_elements = ["circle", "clipPath", "defs", "ellipse", "g", "line", 
        "linearGradient", "mask", "path", "pattern", "polygon", "polyline", "radialGradient",
        "rect", "stop", "svg", "text", "tspan"];
    return valid_react_elements.contains(&test_element)
}

pub fn build_element(name: xml::name::OwnedName,
                 attributes: Vec<xml::attribute::OwnedAttribute>,
                 depth: usize) {

    let temp_name: String = format!("{}", name);
    let svg_tag: String = parse_off_extra_w3c_details(temp_name);
    if valid_react_dom_element(&svg_tag) {
        println!("{}React.DOM.{}", tab_in(depth), svg_tag);
        build_attributes(attributes.clone(), depth);
    }
}
