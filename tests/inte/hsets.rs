use super::utils::{send_command, start_server, stop_server};

#[test]
fn test_hset() {
    let mut server = start_server();

    let response = send_command("HSET myhash name Smith first_name John age 21");
    assert!(response.contains("(integer) 3"));

    let response = send_command("HSET myhash age 34 city Paris");
    assert!(response.contains("(integer) 1"));

    stop_server(&mut server);
}

#[test]
fn test_hget() {
    let mut server = start_server();

    let response = send_command("HSET myhash name Smith first_name John age 21");
    assert!(response.contains("(integer) 3"));

    let response = send_command("HGET myhash name");
    assert!(response.contains("Smith"));

    let response = send_command("HGET myhash first_name");
    assert!(response.contains("John"));

    let response = send_command("HGET myhash age");
    assert!(response.contains("21"));

    let response = send_command("HGET myhash city");
    assert!(response.contains("(nil)"));

    stop_server(&mut server);
}
