pub const ERROR_LIST_KEY: &str =
    "(error) WRONGTYPE Operation against a key holding the wrong kind of value\n";

pub fn format_list_response(data: Vec<String>) -> String {
    data.iter()
        .enumerate()
        .map(|(i, item)| format!("{}) \"{}\"", i + 1, item))
        .collect::<Vec<String>>()
        .join("\n")
        + "\n"
}
