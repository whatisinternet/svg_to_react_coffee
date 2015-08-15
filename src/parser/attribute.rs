extern crate xml;
extern crate case;

use case::CaseExt;
use parser::util::*;

fn valid_react_attribute(test_element: &str) -> bool {
    let valid_react_attributes= [
"width", "height", "style", "clipPath", "cx", "cy", "d", "dx", "dy", "fill", "fillOpacity", "fontFamily",
"fontSize", "fx", "fy", "gradientTransform", "gradientUnits", "markerEnd",
"markerMid", "markerStart", "offset", "opacity", "patternContentUnits",
"patternUnits", "points", "preserveAspectRatio", "r", "rx", "ry",
"spreadMethod", "stopColor", "stopOpacity", "stroke", "strokeDasharray",
"strokeLinecap", "strokeOpacity", "strokeWidth", "textAnchor", "transform",
"version", "viewBox", "x1", "x2", "x", "y1", "y2", "y"
];
    return valid_react_attributes.contains(&test_element)
}



fn transform_attribute (attribute: String) -> String {
    let attribute_key_value: Vec<&str> = attribute.split("=").collect();
    let attribute_key: &str = attribute_key_value[0];
    let attribute_value: &str = attribute_key_value[1];
    return formatted_attribute(attribute_key, attribute_value.clone());
}

fn formatted_attribute ( attribute_key: &str, attribute_value: &str) -> String {
    let mut attributes = "".to_string();
    if valid_react_attribute(attribute_key) {
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
                                    .trim_right_matches("\", ")
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
    }
    return attributes.to_string();
}

pub fn build_attributes(attributes: Vec<xml::attribute::OwnedAttribute>, depth: usize) {
    for attribute in attributes{
        let temp_attribute: String = format!("{}", attribute);
        let parsed_attribute: String = parse_off_extra_w3c_details(temp_attribute);
        let transformed_attribute: String = format!("{}",transform_attribute(parsed_attribute));
        if transformed_attribute != ""{
            println!("{}{}", tab_in(depth + 1), transformed_attribute);
        }
    }
    println!("{},", tab_in(depth +1));
}
