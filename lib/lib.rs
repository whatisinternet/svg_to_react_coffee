extern crate xml;
extern crate case;

mod parser;
mod file_loading;

use parser::parser::*;

#[no_mangle]
pub extern "C" fn convert(svg_file_name: String) {
  let file_name: Vec<&str> = svg_file_name.split(".svg").collect();
  println!("module.exports = React.createFactory React.createClass\n\n  render: ->");
  if file_name.len() > 1 {
    parse_svg(svg_file_name.clone());
  }
}
