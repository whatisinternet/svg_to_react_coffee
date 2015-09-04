pub fn tab_in<'a>(depth: usize) -> String {
    let mut tabs = "".to_string();
    let mut tab_depth = 0;
    while tab_depth <= depth{
        tabs = format!("{}  ",&tabs);
        tab_depth += 1;
    }
    return format!("{}",&tabs);
}

pub fn parse_off_extra_w3c_details<'a> (input_string: String) -> String{
    let string_vector: Vec<&str> = input_string.split("}").collect();
    return format!("{}", string_vector[1]);
}


pub fn valid_react_svg_attribute(test_element: &str) -> bool {
    let valid_react_attributes= [
"clipPath", "cx", "cy", "d", "dx", "dy", "fill", "fillOpacity", "fontFamily",
"fontSize", "fx", "fy", "gradientTransform", "gradientUnits", "markerEnd",
"markerMid", "markerStart", "offset", "opacity", "patternContentUnits",
"patternUnits", "points", "preserveAspectRatio", "r", "rx", "ry",
"spreadMethod", "stopColor", "stopOpacity", "stroke", "strokeDasharray",
"strokeLinecap", "strokeOpacity", "strokeWidth", "textAnchor", "transform",
"version", "viewBox", "x1", "x2", "x", "y1", "y2", "y"
];
    return valid_react_attributes.contains(&test_element) ||
        valid_react_hml_attribute(&test_element)
}

pub fn valid_react_hml_attribute(test_element: &str) -> bool {
    let valid_react_attributes= [
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

pub fn valid_react_dom_svg_element (test_element: &str) -> bool {
    let valid_react_elements = ["circle", "clipPath", "defs", "ellipse", "g", "line",
        "linearGradient", "mask", "path", "pattern", "polygon", "polyline", "radialGradient",
        "rect", "stop", "svg", "text", "tspan"];
    return valid_react_elements.contains(&test_element)
}

pub fn valid_react_dom_html_element (test_element: &str) -> bool {
    let valid_react_elements = ["a", "abbr", "address", "area", "article", "aside", "audio", "b",
    "base", "bdi", "bdo", "big", "blockquote", "body", "br", "button",
    "canvas", "caption", "cite", "code", "col", "colgroup", "data",
    "datalist", "dd", "del", "details", "dfn", "dialog", "div", "dl",
    "dt", "em", "embed", "fieldset", "figcaption", "figure", "footer",
    "form", "h1", "h2", "h3", "h4", "h5", "h6", "head", "header", "hr",
    "html", "i", "iframe", "img", "input", "ins", "kbd", "keygen", "label",
    "legend", "li", "link", "main", "map", "mark", "menu", "menuitem", "meta",
    "meter", "nav", "noscript", "object", "ol", "optgroup", "option",
    "output", "p", "param", "picture", "pre", "progress", "q", "rp", "rt",
    "ruby", "s", "samp", "script", "section", "select", "small", "source",
    "span", "strong", "style", "sub", "summary", "sup", "table", "tbody",
    "td", "textarea", "tfoot", "th", "thead", "time", "title", "tr",
    "track", "u", "ul", "var", "video wbr"];
    return valid_react_elements.contains(&test_element)
}

#[cfg(test)]
mod util_test{
    use parser::util::*;

    #[test]
    fn it_should_tab_in_a_specific_depth() {

        let depth: usize = 0;
        let test_string: String = tab_in(depth);
        let asserted_tabs = 2;

        assert!(asserted_tabs == test_string.len());
    }

    #[test]
    fn it_should_parse_off_any_extra_w3c_details_created_by_xml_parser() {
        let test_string: String =
            parse_off_extra_w3c_details("{w3c}svg".to_string());

        let asserted_string: String =
            "svg".to_string();

        assert!(asserted_string == test_string);
    }

    #[test]
    fn it_should_should_permit_a_valid_html_attribute() {
        assert!(valid_react_hml_attribute("src"));
    }
    #[test]
    fn it_should_should_permit_a_valid_svg_attribute() {
        assert!(valid_react_svg_attribute("x"));
    }

    #[test]
    fn it_should_should_permit_a_valid_data_attribute() {
        assert!(valid_react_svg_attribute("data-something"));
    }

    #[test]
    fn it_should_should_permit_a_valid_aria_attribute() {
        assert!(valid_react_svg_attribute("aria-something"));
    }

    #[test]
    fn it_should_should_reject_an_invalid_attribute() {
        assert!(valid_react_svg_attribute("invalid_attribute") == false);
    }
    #[test]
    fn it_should_should_permit_a_valid_html_element() {
        assert!(valid_react_dom_html_element("html"));
    }

    #[test]
    fn it_should_should_reject_an_invalid_html_element() {
        assert!(valid_react_dom_svg_element("invalid_element") == false);
    }
    #[test]
    fn it_should_should_permit_a_valid_svg_element() {
        assert!(valid_react_dom_svg_element("svg"));
    }

    #[test]
    fn it_should_should_reject_an_invalid_svg_element() {
        assert!(valid_react_dom_svg_element("invalid_element") == false);
    }

}

