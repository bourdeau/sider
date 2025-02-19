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
            SiderResponse::Ok => write!(f, "+OK\r\n"),
            SiderResponse::Int(value) => write!(f, "+(integer) {}\r\n", value),
            SiderResponse::SimpleString(value) => write!(f, "+{}\r\n", value),
            SiderResponse::List(values) => {
                let mut response = format!("*{}\r\n", values.len());

                for value in values {
                    response.push_str(&format!("${}\r\n{}\r\n", value.len(), value));
                }

                write!(f, "{}", response)
            }
            SiderResponse::Nil => write!(f, "+(nil)\r\n"),
            SiderResponse::EmptyArray => write!(f, "+(empty array)\r\n"),
            SiderResponse::Error(err) => write!(f, "-{}\r\n", err),
        }
    }
}
