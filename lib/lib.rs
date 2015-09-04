extern crate xml;
extern crate case;

mod parser;
mod file_loading;

use parser::parser::*;

#[no_mangle]
pub extern "C" fn convert(file_name: String) -> Vec<String> {
  let header_string: String = format!("module.exports = React.createFactory React.createClass\n\n  render: ->");

  if is_svg(file_name.clone()) {
    return parse_svg(file_name.clone(), header_string.clone());

  // } else if is_html(file_name.clone()) {
  //   return parse_html(file_name.clone(), header_string.clone());
  //
  }

  return vec!();
}

    fn convert_svg(svg_file_name: String, header_string: String) -> Vec<String> {
        return parse_svg(svg_file_name.clone(), header_string.clone());
    }

    // fn convert_html(file_name: String, header_string: String) -> Vec<String> {
    //     return parse_html(file_name.clone(), header_string.clone());
    // }

    fn is_svg(file_name: String) -> bool {
        let svg_file_name: Vec<&str> = file_name.split(".svg").collect();
        return svg_file_name.len() > 1
    }

    fn is_html(file_name: String) -> bool {
        let html_file_name: Vec<&str> = file_name.split(".svg").collect();
        return html_file_name.len() > 1
    }
