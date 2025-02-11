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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_list_response_empty() {
        let result = format_list_response(vec![]);
        assert_eq!(result, "\n");
    }

    #[test]
    fn test_format_list_response_single_item() {
        let result = format_list_response(vec!["hello".to_string()]);
        assert_eq!(result, "1) \"hello\"\n");
    }

    #[test]
    fn test_format_list_response_multiple_items() {
        let result = format_list_response(vec!["foo".to_string(), "bar".to_string()]);
        assert_eq!(result, "1) \"foo\"\n2) \"bar\"\n");
    }
}
