pub const ERROR_KEY_TYPE: &str =
    "(error) WRONGTYPE Operation against a key holding the wrong kind of value\n";

pub fn format_list_response(data: Vec<String>) -> String {
    data.iter()
        .enumerate()
        .map(|(i, item)| format!("{}) \"{}\"", i + 1, item))
        .collect::<Vec<String>>()
        .join("\n")
        + "\n"
}
