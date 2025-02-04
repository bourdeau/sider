use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command};
use std::thread::sleep;
use std::time::Duration;

fn start_server() -> Child {
    let child = Command::new("cargo")
        .args(["run"])
        .spawn()
        .expect("Failed to start Redis-like server");

    sleep(Duration::from_secs(1));

    child
}

fn send_command(command: &str) -> String {
    let mut stream = TcpStream::connect("127.0.0.1:6379").expect("Failed to connect to server");
    stream
        .write_all(command.as_bytes())
        .expect("Failed to send command");

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).expect("Failed to read response");

    String::from_utf8_lossy(&buffer[..n]).to_string()
}

#[test]
fn test_basic_set_get() {
    let mut server = start_server();

    let response = send_command("SET name Alice\n");
    assert!(response.contains("OK"));

    let response = send_command("GET name\n");
    assert!(response.contains("Alice"));

    server.kill().expect("Failed to stop server");
}

#[test]
fn test_delete_key() {
    let mut server = start_server();

    send_command("SET city Paris\n");
    let response = send_command("DEL city\n");
    assert!(response.contains("OK"));

    let response = send_command("GET city\n");
    assert!(response.contains("nil"));

    server.kill().expect("Failed to stop server");
}

#[test]
fn test_delete_multiple_keys() {
    let mut server = start_server();
    std::thread::sleep(std::time::Duration::from_secs(1));

    send_command("SET first_name Alice\n");
    send_command("SET last_name Smith\n");
    send_command("SET age 32\n");

    let response = send_command("DEL first_name last_name age\n");

    assert!(response.contains("OK"));

    server.kill().expect("Failed to stop server");
}
