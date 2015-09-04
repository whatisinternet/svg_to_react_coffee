extern crate xml;
use parser::attribute::*;
use parser::util::tab_in;
use parser::util::parse_off_extra_w3c_details;
use parser::util::valid_react_dom_svg_element;

pub fn is_valid_element(name: String) -> bool {
    let temp_name: String = format!("{}", name);
    let svg_tag: String = parse_off_extra_w3c_details(temp_name);
    if valid_react_dom_svg_element(&svg_tag) {
        return true
    }
    return false
}


pub fn build_element(name: String,
                 attributes: Vec<String>,
                 depth: usize) -> Vec<String>{

    let svg_tag: String = parse_off_extra_w3c_details(name);
    let element_with_attributes: Vec<String> = element(svg_tag, attributes, depth);
    return element_with_attributes.clone();
}

    fn element(svg_tag: String,
                           attributes: Vec<String>,
                           depth: usize) -> Vec<String>{
        let mut valid_element_and_attributes: Vec<String> = vec!();
        if valid_react_dom_svg_element(&svg_tag) {
            valid_element_and_attributes = element_and_attributes(svg_tag, attributes, depth);
        }
        return valid_element_and_attributes;
    }

        fn element_and_attributes(svg_tag: String,
                           attributes: Vec<String>,
                           depth: usize) -> Vec<String>{

            let mut dom_element: Vec<String> =
                vec!(format!("{}React.DOM.{}", tab_in(depth), svg_tag));

            let dom_element_attributes: Vec<String> = build_attributes(attributes, depth);
            for attribute in dom_element_attributes {
                dom_element.push(attribute);
            }

            return dom_element;
        }


#[test]
fn it_should_return_valid_dom_elemet_from_element() {
    let mock_svg_element: String = "svg".to_string();
    let mock_svg_attribute: Vec<String> = vec!("{w3c}y2='1438.8'".to_string());
    let mock_depth: usize = 1;

    let asserted_vector: Vec<String> = element(mock_svg_element,
                                                mock_svg_attribute,
                                                mock_depth);

    let expected_element_string: String =
        format!("{}{}", tab_in(mock_depth), "React.DOM.svg".to_string());

    assert!( asserted_vector[0].to_string() == expected_element_string);
}

#[test]
fn it_should_return_a_vector_of_strings_with_length_3() {
    let mock_svg_element: String = "svg".to_string();
    let mock_svg_attribute: Vec<String> = vec!("{w3c}y2='1438.8'".to_string());
    let mock_depth: usize = 1;

    let asserted_vector: Vec<String> = element_and_attributes(mock_svg_element,
                                                            mock_svg_attribute,
                                                            mock_depth);

    assert!( asserted_vector.len() == 3);
}

#[test]
fn it_should_return_valid_dom_elemet() {
    let mock_svg_element: String = "svg".to_string();
    let mock_svg_attribute: Vec<String> = vec!("{w3c}y2='1438.8'".to_string());
    let mock_depth: usize = 1;

    let asserted_vector: Vec<String> = element_and_attributes(mock_svg_element,
                                                            mock_svg_attribute,
                                                            mock_depth);

    let expected_element_string: String =
        format!("{}{}", tab_in(mock_depth), "React.DOM.svg".to_string());

    assert!( asserted_vector[0].to_string() == expected_element_string);
}
#[test]
fn it_should_return_an_empty_vector() {
    let mock_svg_element: String = "invalid".to_string();
    let mock_svg_attribute: Vec<String> = vec!("{w3c}y2='1438.8'".to_string());
    let mock_depth: usize = 1;

    let asserted_vector: Vec<String> = element(mock_svg_element,
                                                mock_svg_attribute,
                                                mock_depth);

    assert!( asserted_vector.len() == 0);
}


// Test to move into /test/
#[cfg(test)]
mod util_test{
    use parser::element::*;
    use parser::util::tab_in;

    #[test]
    fn it_should_return_true_on_valid_element () {
        let temp_name: String = "{w3c}svg".to_string();
        let valid_element: bool = is_valid_element(temp_name);
        assert!(valid_element);
    }

    #[test]
    fn it_should_return_false_on_an_invalid_element () {
        let temp_name: String = "{w3c}INVALID".to_string();
        let invalid_element: bool = is_valid_element(temp_name);
        assert!(invalid_element == false);
    }

    #[test]
    fn it_should_return_valid_dom_elemet_from_build_element() {
        let mock_svg_element: String = "{w3c}svg".to_string();
        let mock_svg_attribute: Vec<String> = vec!("{w3c}y2='1438.8'".to_string());
        let mock_depth: usize = 1;

        let asserted_vector: Vec<String> = build_element(mock_svg_element,
                                                    mock_svg_attribute,
                                                    mock_depth);

        let expected_element_string: String =
            format!("{}{}", tab_in(mock_depth), "React.DOM.svg".to_string());

        assert!( asserted_vector[0].to_string() == expected_element_string);
    }

}
