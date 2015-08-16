extern crate xml;
extern crate case;

use case::CaseExt;
use parser::util::*;

fn valid_react_attribute(test_element: &str) -> bool {
    let valid_react_attributes= [
"clipPath", "cx", "cy", "d", "dx", "dy", "fill", "fillOpacity", "fontFamily",
"fontSize", "fx", "fy", "gradientTransform", "gradientUnits", "markerEnd",
"markerMid", "markerStart", "offset", "opacity", "patternContentUnits",
"patternUnits", "points", "preserveAspectRatio", "r", "rx", "ry",
"spreadMethod", "stopColor", "stopOpacity", "stroke", "strokeDasharray",
"strokeLinecap", "strokeOpacity", "strokeWidth", "textAnchor", "transform",
"version", "viewBox", "x1", "x2", "x", "y1", "y2", "y",
"accept", "acceptCharset", "accessKey", "action", "allowFullScreen",
"allowTransparency", "alt", "async", "autoComplete", "autoFocus", "autoPlay",
"cellPadding", "cellSpacing", "charSet", "checked", "classID", "className",
"colSpan", "cols", "content", "contentEditable", "contextMenu", "controls",
"coords", "crossOrigin", "data", "dateTime", "defer", "dir", "disabled",
"download", "draggable", "encType", "form", "formAction", "formEncType",
"formMethod", "formNoValidate", "formTarget", "frameBorder", "headers",
"height", "hidden", "high", "href", "hrefLang", "htmlFor", "httpEquiv", "icon",
"id", "label", "lang", "list", "loop", "low", "manifest", "marginHeight",
"marginWidth", "max", "maxLength", "media", "mediaGroup", "method", "min",
"multiple", "muted", "name", "noValidate", "open", "optimum", "pattern",
"placeholder", "poster", "preload", "radioGroup", "readOnly", "rel", "required",
"role", "rowSpan", "rows", "sandbox", "scope", "scoped", "scrolling",
"seamless", "selected", "shape", "size", "sizes", "span", "spellCheck", "src",
"srcDoc", "srcSet", "start", "step", "style", "tabIndex", "target", "title",
"type", "useMap", "value", "width", "wmode"
];
    return valid_react_attributes.contains(&test_element) ||
        test_element.starts_with("data") ||
        test_element.starts_with("aria");
}



fn transform_attribute (attribute: String) -> String {
    let attribute_key_value: Vec<&str> = attribute.split("=").collect();
    let attribute_key: &str = attribute_key_value[0];
    let attribute_value: &str = attribute_key_value[1];
    return formatted_attribute(attribute_key, attribute_value.clone());
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

fn parse_valid_attribute( attribute_key: &str, attribute_value: &str) -> String {
    if is_style_attribute(attribute_key) {
        return  parse_style_attribute(attribute_key, attribute_value);
    } else if !attribute_key.contains(":"){
        return parse_normal_attribute(attribute_key, attribute_value);
    }
    return "".to_string();
}

fn formatted_attribute ( attribute_key: &str, attribute_value: &str) -> String {
    let mut attributes = "".to_string();
    let temp_attribute = attribute_key.replace("-","_").to_camel_lowercase().to_string();
    if valid_react_attribute(&temp_attribute) {
        return parse_valid_attribute(attribute_key, attribute_value);
   }
    return attributes.to_string();
}

fn print_valid_attribute( depth: usize, attribute: xml::attribute::OwnedAttribute) {
    let temp_attribute: String = format!("{}", attribute);
    let parsed_attribute: String = parse_off_extra_w3c_details(temp_attribute);
    let transformed_attribute: String = format!("{}",transform_attribute(parsed_attribute));
    if transformed_attribute != ""{
        println!("{}{}", tab_in(depth + 1), transformed_attribute);
    }
}

pub fn build_attributes(attributes: Vec<xml::attribute::OwnedAttribute>, depth: usize) {
    for attribute in attributes{
        print_valid_attribute( depth, attribute);
    }
    println!("{},", tab_in(depth +1));
}
