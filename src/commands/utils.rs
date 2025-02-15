pub fn format_single_response(data: &str) -> String {
    format!("\"{}\"\n", data)
}

pub fn format_int(data: i64) -> String {
    format!("(integer) {}\n", data)
}

pub fn format_list_response(data: Vec<String>) -> String {
    data.iter()
        .enumerate()
        .map(|(i, item)| format!("{}) \"{}\"", i + 1, item))
        .collect::<Vec<String>>()
        .join("\n")
        + "\n"
}
