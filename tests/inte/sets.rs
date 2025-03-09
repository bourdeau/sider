use super::utils::{send_command, start_server, stop_server};

#[test]
fn test_sadd() {
    let mut server = start_server();

    let response = send_command("SADD myset Hello World");
    assert!(response.contains("(integer) 2"));

    let response = send_command("SADD myset How are you");
    assert!(response.contains("(integer) 5"));

    // World is already in the set
    let response = send_command("SADD myset World");
    assert!(response.contains("(integer) 5"));

    stop_server(&mut server);
}

#[test]
fn test_smembers() {
    let mut server = start_server();

    let response = send_command("SADD myset Hello World You Are Lovely");
    assert!(response.contains("(integer) 5"));

    let response = send_command("SMEMBERS myset");
    assert!(response.contains("Hello"));
    assert!(response.contains("World"));
    assert!(response.contains("You"));
    assert!(response.contains("Are"));
    assert!(response.contains("Lovely"));

    let response = send_command("SMEMBERS non_existing_set");
    assert!(response.contains("(empty array)"));

    stop_server(&mut server);
}
