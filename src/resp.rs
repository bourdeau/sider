use crate::errors::SiderError;

pub fn parse_resp_command(resp_command: &str) -> Result<Vec<String>, SiderError> {
    let mut lines = resp_command.split_terminator("\r\n");

    let first_line = lines.next().ok_or(SiderError::EmptyRequest)?;

    if !first_line.starts_with('*') {
        return Err(SiderError::InvalidArrayPrefix);
    }

    let nb_elements: usize = first_line[1..]
        .parse()
        .map_err(|_| SiderError::InvalidArrayLength)?;

    let mut command = Vec::new();

    for line in lines {
        if line.starts_with('$') {
            continue;
        }
        command.push(line.to_string());
    }

    if command.len() != nb_elements {
        return Err(SiderError::WrongElementCount);
    }

    if command[0] == "COMMAND" {
        command.remove(0);
    }

    Ok(command)
}
