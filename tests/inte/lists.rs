use super::utils::{send_command, start_server, stop_server};

#[test]
fn test_lpush() {
    let mut server = start_server();

    let response = send_command("LPUSH names Alice Bob Charlie");
    assert!(response.contains("(integer) 3"));

    let response = send_command("LPUSH names David");
    assert!(response.contains("(integer) 4"));

    let response = send_command("LPUSH names Eve");
    assert!(response.contains("(integer) 5"));

    let response = send_command("LPUSH names Eve");
    assert!(response.contains("(integer) 6"));

    let response = send_command("GET names");
    assert!(response
        .contains("(error) WRONGTYPE Operation against a key holding the wrong kind of value"));

    stop_server(&mut server);
}

#[test]
fn test_expire() {
    let mut server = start_server();

    send_command("LPUSH list:expire Alice Bob Charlie");
    let response = send_command("EXPIRE list:expire 3");
    assert!(response.contains("(integer) 1"));

    stop_server(&mut server);
}

#[test]
fn test_rpush() {
    let mut server = start_server();

    let response = send_command("RPUSH names Alice Bob Charlie");
    assert!(response.contains("(integer) 3"));

    let response = send_command("RPUSH names David");
    assert!(response.contains("(integer) 4"));

    let response = send_command("RPUSH names Eve");
    assert!(response.contains("(integer) 5"));

    let response = send_command("RPUSH names Eve");
    assert!(response.contains("(integer) 6"));

    let response = send_command("GET names");
    assert!(response
        .contains("(error) WRONGTYPE Operation against a key holding the wrong kind of value"));

    stop_server(&mut server);
}

#[test]
fn test_lrange() {
    let mut server = start_server();

    let response = send_command("LPUSH mylist C B A");
    assert!(response.contains("(integer) 3"));

    let response = send_command("LRANGE mylist 0 -1");
    assert!(response.contains("1) \"A\""));
    assert!(response.contains("2) \"B\""));
    assert!(response.contains("3) \"C\""));

    let response = send_command("LRANGE mylist 1 2");
    assert!(response.contains("1) \"B\""));
    assert!(response.contains("2) \"C\""));

    let response = send_command("LRANGE mylist -1 -1");
    assert!(response.contains("1) \"C\""));

    let response = send_command("LRANGE mylist 10 20");
    assert!(response.contains("(empty array)"));

    let response = send_command("LRANGE unknownkey 0 -1");
    assert!(response.contains("(empty array)"));

    send_command("SET notalist 123");
    let response = send_command("LRANGE notalist 0 -1");
    assert!(response
        .contains("(error) WRONGTYPE Operation against a key holding the wrong kind of value"));

    stop_server(&mut server);
}

#[test]
fn test_lpop() {
    let mut server = start_server();

    let response = send_command("LPUSH lpoplist A B C D E");
    assert!(response.contains("(integer) 5"));

    let response = send_command("LPOP lpoplist");
    assert!(response.contains("\"E\""));

    let response = send_command("LPOP lpoplist 2");
    assert!(response.contains("1) \"D\""));
    assert!(response.contains("2) \"C\""));

    let response = send_command("LPOP lpoplist 10");
    assert!(response.contains("1) \"B\""));
    assert!(response.contains("2) \"A\""));

    let response = send_command("LPOP lpoplist");
    assert!(response.contains("(nil)"));

    stop_server(&mut server);
}

#[test]
fn test_rpop() {
    let mut server = start_server();

    let response = send_command("RPUSH rpoplist A B C");
    assert!(response.contains("(integer) 3"));

    let response = send_command("RPOP rpoplist");
    assert!(response.contains("\"C\""));

    let response = send_command("RPOP rpoplist 2");
    assert!(response.contains("1) \"B\""));
    assert!(response.contains("2) \"A\""));

    let response = send_command("RPOP rpoplist");
    assert!(response.contains("(nil)"));

    stop_server(&mut server);
}
