use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command};
use std::thread::sleep;
use std::time::Duration;

pub fn start_server() -> Child {
    let child = Command::new("cargo")
        .args(["run"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to start Redis-like server");

    // Actively check if the server is ready before continuing
    for _ in 0..20 {
        if TcpStream::connect("127.0.0.1:6379").is_ok() {
            send_command("FLUSHDB");
            return child;
        }
        sleep(Duration::from_secs(1)); // Wait before retrying
    }

    panic!("Server did not start in time");
}

pub fn stop_server(server: &mut Child) {
    if let Err(e) = server.kill() {
        eprintln!("Warning: Failed to kill server: {:?}", e);
    }
    let _ = server.wait(); // Ensure process cleanup
    sleep(Duration::from_secs(1)); // Give OS time to release the port
}

pub fn send_command(command: &str) -> String {
    let mut stream = TcpStream::connect("127.0.0.1:6379").expect("Failed to connect to server");
    stream
        .write_all(command.as_bytes())
        .expect("Failed to send command");

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).expect("Failed to read response");

    String::from_utf8_lossy(&buffer[..n]).to_string()
}
