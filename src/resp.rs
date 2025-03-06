use crate::errors::SiderError;

use regex::Regex;

pub fn parse_resp_command(resp_command: &str) -> Result<Vec<Vec<String>>, SiderError> {
    let lines = resp_command.split_terminator("\r\n");
    let mut commands: Vec<Vec<String>> = Vec::new();
    let mut cmd_nb: Option<usize> = None;
    let resp_array_regex = Regex::new(r"^\*\d+").expect("Regex error");

    for line in lines {
        if resp_array_regex.is_match(line) {
            commands.push(Vec::new());
            cmd_nb = Some(commands.len() - 1);
            continue;
        }

        if line.starts_with('$') {
            continue;
        }

        if line.starts_with("COMMAND") {
            continue;
        }

        if let Some(idx) = cmd_nb {
            commands[idx].push(line.to_string());
        }
    }

    println!("{:?}", commands);

    Ok(commands)
}
