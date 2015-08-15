pub fn tab_in<'a>(depth: usize) -> String {
    let mut tabs = "".to_string();
    for i in (1..).take(depth){
        tabs = format!("{}  ",&tabs);
    }
    return format!("{}",&tabs);
}

pub fn parse_off_extra_w3c_details<'a> (input_string: String) -> String{
    let string_vector: Vec<&str> = input_string.split("}").collect();
    return format!("{}", string_vector[1]);
}
