extern crate xml;
extern crate case;

use case::CaseExt;
use parser::util::tab_in;
use parser::util::parse_off_extra_w3c_details;
use parser::util::valid_react_svg_attribute;

pub fn build_attributes(
    attributes: Vec<String>,
    depth: usize) -> Vec<String> {

    let mut element_attributes: Vec<String> = vec!();
    for attribute in attributes{
        let valid_attribute: String = valid_attribute( depth, attribute ).to_string();
        if valid_attribute != "".to_string(){
            element_attributes.push(valid_attribute);
        }
    }
    element_attributes.push( format!("{},", tab_in(depth +1)));
    return element_attributes;
}

    fn valid_attribute( depth: usize, attribute: String) -> String {
        let temp_attribute: String = format!("{}", attribute);
        let parsed_attribute: String = parse_off_extra_w3c_details(temp_attribute);
        let transformed_attribute: String = format!("{}",transform_attribute(parsed_attribute));
        if transformed_attribute != ""{
            return format!("{}{}", tab_in(depth + 1), transformed_attribute).to_string();
        }
    return "".to_string();
    }


        fn transform_attribute (attribute: String) -> String {
            let attribute_key_value: Vec<&str> = attribute.split("=").collect();
            let attribute_key: &str = attribute_key_value[0];
            let attribute_value: &str = attribute_key_value[1];
            return formatted_attribute(attribute_key, attribute_value.clone());
        }


            fn formatted_attribute ( attribute_key: &str, attribute_value: &str) -> String {
                let temp_attribute = attribute_key.replace("-","_").to_camel_lowercase().to_string();
                if valid_react_svg_attribute(&temp_attribute) {
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
                            return format!("{}", attribute_value
                                .replace(",","")
                                .replace("-","_")
                                .replace("\"","")
                                .replace(":",": \"")
                                .replace(";","\", ")
                                .trim_right_matches("\", ")
                                .to_string()
                                .to_camel_lowercase());
                        }

#[test]
fn it_should_build_a_vector_of_valid_attributes() {
    let mock_depth: usize = 0;
    let mock_attribute: String = "{w3c}width=baz".to_string();
    let mock_attributes: Vec<String> = vec!(mock_attribute);
    let expected_string: String = "    width: baz".to_string();
    let expected_vector: Vec<String> = vec!(expected_string);
    let asserted_vector: Vec<String> = build_attributes(mock_attributes, mock_depth);

    for n in 0..(asserted_vector.len() - 1)  {
        assert_eq!( asserted_vector[n], expected_vector[n]);
    }
}

#[test]
fn it_should_return_an_empty_string_from_valid_attribute_when_given_invalid_data() {
    let mock_depth: usize = 0;
    let mock_attribute: String = "{w3c}foo=baz".to_string();
    let expected_string: String = "".to_string();
    let asserted_string: String = valid_attribute(mock_depth, mock_attribute);

    assert!( asserted_string == expected_string);
}

#[test]
fn it_should_return_a_properly_formatted_attribute_from_valid_attribute() {
    let mock_depth: usize = 0;
    let mock_attribute: String = "{w3c}width=baz".to_string();
    let expected_string: String = "    width: baz".to_string();
    let asserted_string: String = valid_attribute(mock_depth, mock_attribute);

    assert!( asserted_string == expected_string);
}

#[test]
fn it_should_return_a_valid_attribute_when_a_valid_attribute_transform() {
    let mock_attribute: String = "width=baz".to_string();
    let expected_string: String = "width: baz".to_string();
    let asserted_string: String = transform_attribute(mock_attribute);

    assert!( asserted_string == expected_string);
}

#[test]
fn it_should_return_nothing_when_an_invalid_attribute_when_given_a_valid_attribute_transform() {
    let mock_attribute: String = "foo=baz".to_string();
    let expected_string: String = "".to_string();
    let asserted_string: String = transform_attribute(mock_attribute);

    assert!( asserted_string == expected_string);
}

#[test]
fn it_should_return_nothing_when_an_invalid_attribute_when_given_a_valid_attribute() {
    let mock_key: &str = "foo";
    let mock_value: &str = "baz";
    let expected_string: String = "".to_string();
    let asserted_string: String = formatted_attribute(mock_key, mock_value);

    assert!( asserted_string == expected_string);
}

#[test]
fn it_should_return_formatted_attribute_when_given_a_valid_attribute() {
    let mock_key: &str = "width";
    let mock_value: &str = "baz";
    let expected_string: String = "width: baz".to_string();
    let asserted_string: String = formatted_attribute(mock_key, mock_value);

    assert!( asserted_string == expected_string);
}

#[test]
fn it_should_return_nothing_on_invalid_attribute_non_style() {
    let mock_key: &str = "foo-bar:";
    let mock_value: &str = "baz";
    let expected_string: String = "".to_string();
    let asserted_string: String = formatted_attribute(mock_key, mock_value);

    assert!( asserted_string == expected_string);
}

#[test]
fn it_should_return_a_parsed_attribute_style() {
    let mock_key: &str = "style";
    let mock_value: &str = "bar-baz:\"baz-foo\";";
    let expected_string: String = "style: { barBaz: \"bazFoo\" }".to_string();
    let asserted_string: String = parse_attribute(mock_key, mock_value);

    assert!( asserted_string == expected_string);
}

#[test]
fn it_should_return_a_parsed_attribute_non_style() {
    let mock_key: &str = "foo-bar";
    let mock_value: &str = "baz";
    let expected_string: String = "fooBar: baz".to_string();
    let asserted_string: String = parse_attribute(mock_key, mock_value);

    assert!( asserted_string == expected_string);
}

#[test]
fn it_should_return_true_on_valid_style_key() {
    let mock_key: &str = "style";
    let asserted_bool: bool = is_style_attribute(mock_key);

    assert!( asserted_bool == true);
}

#[test]
fn it_should_return_false_on_fake_style_key() {
    let mock_key: &str = "foo-bar";
    let asserted_bool: bool = is_style_attribute(mock_key);

    assert!( asserted_bool == false);
}

#[test]
fn it_should_return_a_parsed_style_attribute() {
    let mock_key: &str = "foo-bar";
    let mock_value: &str = "bar-baz:\"baz-foo\";";
    let expected_string: String = "fooBar: { barBaz: \"bazFoo\" }".to_string();
    let asserted_string: String = parse_style_attribute(mock_key, mock_value);

    assert!( asserted_string == expected_string);
}

#[test]
fn it_should_return_a_parsed_key_value_attribute() {
    let mock_key: &str = "foo-bar";
    let mock_value: &str = "baz";
    let expected_string: String = "fooBar: baz".to_string();
    let asserted_string: String = parse_normal_attribute(mock_key, mock_value);

    assert!( asserted_string == expected_string);
}

#[test]
fn it_should_return_a_parsed_attribute() {
    let mock_string: &str = "foo-bar:\"baz\";";
    let expected_string: String = "fooBar: \"baz".to_string();
    let asserted_string: String = parse_attribute_value(mock_string);

    assert!( asserted_string == expected_string);
}
