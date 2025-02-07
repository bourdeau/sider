use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command};
use std::thread::sleep;
use std::time::Duration;

fn start_server() -> Child {
    let child = Command::new("cargo")
        .args(["run"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to start Redis-like server");

    // Actively check if the server is ready before continuing
    for _ in 0..20 {
        if TcpStream::connect("127.0.0.1:6379").is_ok() {
            return child;
        }
        sleep(Duration::from_millis(200)); // Wait before retrying
    }

    panic!("Server did not start in time");
}

fn stop_server(server: &mut Child) {
    if let Err(e) = server.kill() {
        eprintln!("Warning: Failed to kill server: {:?}", e);
    }
    let _ = server.wait(); // Ensure process cleanup
    sleep(Duration::from_secs(1)); // Give OS time to release the port
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

    stop_server(&mut server);
}

#[test]
fn test_delete_key() {
    let mut server = start_server();

    send_command("SET city Paris");
    let response = send_command("DEL city");
    assert!(response.contains("OK"));

    let response = send_command("GET city");
    assert!(response.contains("nil"));

    stop_server(&mut server);
}

#[test]
fn test_delete_multiple_keys() {
    let mut server = start_server();

    send_command("SET first_name Alice");
    send_command("SET last_name Smith");
    send_command("SET age 32");

    let response = send_command("DEL first_name last_name age");

    assert!(response.contains("OK"));

    stop_server(&mut server);
}

#[test]
fn test_key_regex() {
    let mut server = start_server();

    send_command("SET first_name Alice");
    send_command("SET last_name Smith");
    send_command("SET age 32\n");

    let response = send_command("KEYS *");
    assert!(response.contains("first_name"));
    assert!(response.contains("last_name"));

    let response = send_command("KEYS first*");
    assert!(response.contains("first_name"));

    let response = send_command("KEYS *name*");
    assert!(response.contains("first_name"));
    assert!(response.contains("last_name"));

    let response = send_command("KEYS f?rst_name");
    assert!(response.contains("first_name"));

    send_command("FLUSHDB");

    let response = send_command("KEYS *");
    assert!(response.contains("(empty array)"));

    stop_server(&mut server);
}

#[test]
fn test_exists() {
    let mut server = start_server();

    send_command("SET first_name Alice");
    send_command("SET last_name Smith");
    send_command("SET age 32");

    let response = send_command("EXISTS first_name");
    assert!(response.contains("1"));

    let response = send_command("EXISTS middle_name");
    assert!(response.contains("0"));

    let response = send_command("EXISTS first_name last_name middle_name");
    assert!(response.contains("2"));

    let response = send_command("EXISTS first_name last_name age");
    assert!(response.contains("3"));

    stop_server(&mut server);
}

#[test]
fn test_expire() {
    let mut server = start_server();

    send_command("SET name Smith");
    let response = send_command("EXPIRE name 3");
    assert!(response.contains("(integer) 1"));

    stop_server(&mut server);
}

#[test]
fn test_ttl() {
    let mut server = start_server();

    send_command("SET name Smith");
    send_command("EXPIRE name 3");

    std::thread::sleep(std::time::Duration::from_secs(1));

    // (integer) 2
    let ttl = send_command("TTL name");

    let ttl_int: i32 = ttl
        .split_whitespace()
        .last()
        .and_then(|s| s.parse::<i32>().ok())
        .expect("Failed to parse TTL value");

    assert!(ttl_int < 3);

    stop_server(&mut server);
}

#[test]
fn test_background_delete() {
    let mut server = start_server();

    send_command("SET name Smith");
    send_command("EXPIRE name 10");

    // Background delete occurs every 60 secs
    std::thread::sleep(std::time::Duration::from_secs(70));

    let response = send_command("EXISTS name");
    assert!(response.contains("0"));

    stop_server(&mut server);
}