use crate::errors::SiderError;
use std::fmt;

#[derive(Debug, Clone)]
pub enum SiderResponse {
    Ok,                   // "OK"
    Int(i64),             // "(integer) 123"
    SimpleString(String), // "foo"
    List(Vec<String>),    // "1) foo\n2) bar\n"
    Nil,                  // "(nil)"
    EmptyArray,           // "(empty array)"
    Error(SiderError),    // Handles errors gracefully
}

impl fmt::Display for SiderResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SiderResponse::Ok => writeln!(f, "\"OK\""),
            SiderResponse::Int(value) => writeln!(f, "(integer) {}", value),
            SiderResponse::SimpleString(value) => writeln!(f, "\"{}\"", value),
            SiderResponse::List(values) => {
                let formatted = values
                    .iter()
                    .enumerate()
                    .map(|(i, v)| format!("{}) \"{}\"", i + 1, v))
                    .collect::<Vec<_>>()
                    .join("\n");
                writeln!(f, "{}", formatted)
            }
            SiderResponse::Nil => writeln!(f, "(nil)"),
            SiderResponse::EmptyArray => writeln!(f, "(empty array)"),
            SiderResponse::Error(err) => writeln!(f, "{}", err),
        }
    }
}
