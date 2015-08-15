extern crate xml;
extern crate case;

use std::env;

mod parser;
mod file_loading;

use parser::parser::*;

fn main() {
  let svg_file: Vec<_> = env::args().collect();
  let file_name: Vec<&str> = svg_file[1].split(".svg").collect();
  println!("module.exports = React.createFactory React.createClass\n\n  render: ->");
  if file_name.len() > 1 {
    parse_svg(svg_file[1].clone());
  }
}
