mod utils;
use utils::{send_command, start_server, stop_server};

#[test]
fn test_basic_set_get() {
    let mut server = start_server();

    let response = send_command("SET name Alice");
    assert!(response.contains("OK"));

    let response = send_command("GET name");
    assert!(response.contains("Alice"));

    stop_server(&mut server);
}

#[test]
fn test_delete_key() {
    let mut server = start_server();

    send_command("SET city Paris");
    let response = send_command("DEL city");
    assert!(response.contains("(integer) 1"));

    let response = send_command("GET city");
    assert!(response.contains("(nil)"));

    stop_server(&mut server);
}

#[test]
fn test_delete_multiple_keys() {
    let mut server = start_server();

    send_command("SET first_name Alice");
    send_command("SET last_name Smith");
    send_command("SET age 32");

    let response = send_command("DEL first_name last_name age");

    assert!(response.contains("(integer) 3"));

    stop_server(&mut server);
}

#[test]
fn test_key_regex() {
    let mut server = start_server();

    send_command("SET first_name Alice");
    send_command("SET last_name Smith");
    send_command("SET age 32");

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
    std::thread::sleep(std::time::Duration::from_secs(10));

    let response = send_command("KEYS *");
    assert!(
        response.contains("(empty array)"),
        "Test failed! Actual response: {:?}",
        response
    );

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

#[test]
fn test_incr() {
    let mut server = start_server();

    // Create a key if it doesn't exist
    let response = send_command("INCR counter");
    assert!(response.contains("(integer) 1"));

    // Increment the key
    let response = send_command("INCR counter");
    assert!(response.contains("(integer) 2"));

    let response = send_command("GET counter");
    assert!(response.contains("2"));

    stop_server(&mut server);
}

#[test]
fn test_decr() {
    let mut server = start_server();

    // Create a key if it doesn't exist
    let response = send_command("DECR another_counter");
    assert!(response.contains("(integer) -1"));

    // Decrement the key
    let response = send_command("DECR another_counter");
    assert!(response.contains("(integer) -2"));

    let response = send_command("GET another_counter");
    assert!(response.contains("-2"));

    stop_server(&mut server);
}

#[test]
fn test_incrby() {
    let mut server = start_server();

    // Create a key if it doesn't exist
    let response = send_command("INCRBY incrby 5");
    assert!(response.contains("(integer) 5"));

    // Increment the key by 10
    let response = send_command("INCRBY incrby 10");
    assert!(response.contains("(integer) 15"));

    let response = send_command("GET incrby");
    assert!(response.contains("15"));

    // Decrement the key by 100
    let response = send_command("INCRBY incrby -100");
    assert!(response.contains("(integer) -85"));

    stop_server(&mut server);
}

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

    let response = send_command("LPUSH mylist A B C D E");
    assert!(response.contains("(integer) 5"));

    let response = send_command("LPOP mylist");
    assert!(response.contains("1 \"E\""));

    let response = send_command("LPOP mylist 2");
    assert!(response.contains("1 \"D\""));
    assert!(response.contains("2 \"C\""));

    let response = send_command("LPOP mylist 10");
    assert!(response.contains("1 \"B\""));
    assert!(response.contains("2 \"A\""));

    let response = send_command("LPOP mylist");
    assert!(response.contains("(nil)"));

    stop_server(&mut server);
}
