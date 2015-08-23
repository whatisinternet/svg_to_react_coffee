extern crate xml;
extern crate case;

mod parser;
mod file_loading;

use parser::parser::*;

#[no_mangle]
pub extern "C" fn convert(svg_file_name: String) -> Vec<String> {
  let file_name: Vec<&str> = svg_file_name.split(".svg").collect();
  let header_string: String = format!("module.exports = React.createFactory React.createClass\n\n  render: ->");
  if file_name.len() > 1 {
    return parse_svg(svg_file_name.clone(), header_string.clone());
  }
  return vec!();
}
