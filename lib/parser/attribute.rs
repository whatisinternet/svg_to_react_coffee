extern crate xml;
extern crate case;

use case::CaseExt;
use parser::util::tab_in;
use parser::util::parse_off_extra_w3c_details;
use parser::util::valid_react_attribute;

pub fn build_attributes(attributes: Vec<xml::attribute::OwnedAttribute>, depth: usize) {
    for attribute in attributes{
        print_valid_attribute( depth, attribute )
    }
    println!("{},", tab_in(depth +1));
}

    fn print_valid_attribute( depth: usize, attribute: xml::attribute::OwnedAttribute) {
        let temp_attribute: String = format!("{}", attribute);
        let parsed_attribute: String = parse_off_extra_w3c_details(temp_attribute);
        let transformed_attribute: String = format!("{}",transform_attribute(parsed_attribute));
        if transformed_attribute != ""{
            println!("{}{}", tab_in(depth + 1), transformed_attribute);
        }
    }


        fn transform_attribute (attribute: String) -> String {
            let attribute_key_value: Vec<&str> = attribute.split("=").collect();
            let attribute_key: &str = attribute_key_value[0];
            let attribute_value: &str = attribute_key_value[1];
            return formatted_attribute(attribute_key, attribute_value.clone());
        }


            fn formatted_attribute ( attribute_key: &str, attribute_value: &str) -> String {
                let temp_attribute = attribute_key.replace("-","_").to_camel_lowercase().to_string();
                if valid_react_attribute(&temp_attribute) {
                    return parse_attribute(attribute_key, attribute_value);
            }
                return "".to_string();
            }

                fn parse_attribute( attribute_key: &str, attribute_value: &str) -> String {
                    if is_style_attribute(attribute_key) {
                        return  parse_style_attribute(attribute_key, attribute_value);
                    } else if !attribute_key.contains(":"){
                        return parse_normal_attribute(attribute_key, attribute_value);
                    }
                    return "".to_string();
                }

                    fn is_style_attribute (attribute_key: &str) -> bool {
                        return  attribute_key == "style"
                    }

                    fn parse_style_attribute( attribute_key: &str, attribute_value: &str) -> String {
                        return format!("{}: {{ {}\" }}",
                                            attribute_key
                                            .replace("-","_")
                                            .to_camel_lowercase(),
                                            parse_attribute_value(attribute_value));
                    }

                    fn parse_normal_attribute( attribute_key: &str, attribute_value: &str) -> String {
                        return format!("{}: {}",
                                            attribute_key
                                            .replace("-","_")
                                            .to_camel_lowercase(),
                                            attribute_value);
                    }
                        fn parse_attribute_value ( attribute_value: &str) -> String {
                            return attribute_value
                                .replace(",","")
                                .replace("-","_")
                                .replace("\"","")
                                .replace(":",": \"")
                                .replace(";","\", ")
                                .trim_right_matches("\", ")
                                .to_string()
                                .to_camel_lowercase();
                        }






