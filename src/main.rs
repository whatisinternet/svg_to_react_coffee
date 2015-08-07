use std::env;

fn load_svg (fila_name: String) {
  
}


fn main() {
  let svg_file: Vec<_> = env::args().collect();
  let file_name: Vec<&str> = svg_file[1].split(".svg").collect();
  if file_name.len() > 1 {
    load_svg(svg_file[1])
  }
}
