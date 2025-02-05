# 🐆 Sider

![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/bourdeau/sider) [![build status](https://github.com/bourdeau/sider/actions/workflows/build.yml/badge.svg)](https://github.com/bourdeau/sider/actions) [![dependency status](https://deps.rs/repo/github/bourdeau/sider/status.svg)](https://deps.rs/repo/github/bourdeau/sider)

Sider is a Redis-compatible server reimplemented from scratch in Rust. It aims to provide a high-performance, in-memory key-value store while leveraging Rust’s safety and concurrency features. Currently, Sider is only supported on Linux, with potential cross-platform support planned for the future.


## Setup

Run the sider server:

```bash
cargo run
```
In another terminal:

```
nc 127.0.0.1 6379
SET first_name John
```

## Features

### Commands

#### Key-Value Storage (Essential for Caching, Sessions)

| Command  | Syntax | Example | Output | To implement | Done |
|----------|--------|---------|--------|-------------|------|
| **SET**  | `SET key value` | `SET user:1 "John"` | `OK` |  | ✅ |
| **GET**  | `GET key` | `GET user:1` | `"John"` |  | ✅ |
| **DEL**  | `DEL key` | `DEL user:1` | `1` (if key existed) |  | ✅ |
| **EXISTS** | `EXISTS key` | `EXISTS user:1` | `1` (exists) / `0` (not) | ✅ |   |


#### Expiration & Time-to-Live (For Caching, Sessions)

| Command  | Syntax | Example | Output | To implement | Done |
|----------|--------|---------|--------|-------------|------|
| **EXPIRE** | `EXPIRE key seconds` | `EXPIRE user:1 3600` | `1` (success) | ✅ |   |
| **TTL**  | `TTL key` | `TTL user:1` | `3599` (seconds left) | ✅ |   |


#### Counters & Rate Limiting (For Throttling, Analytics)

| Command  | Syntax | Example | Output | To implement | Done |
|----------|--------|---------|--------|-------------|------|
| **INCR**  | `INCR key` | `INCR api:requests` | `1`, `2`, `3`... | ✅ |   |
| **DECR**  | `DECR key` | `DECR api:requests` | `2`, `1`, `0`... | ✅ |   |
| **INCRBY** | `INCRBY key amount` | `INCRBY api:requests 5` | `5`, `10`, `15`... | ✅ |   |


#### Lists (For Background Jobs, Notifications, Queues)

| Command  | Syntax | Example | Output | To implement | Done |
|----------|--------|---------|--------|-------------|------|
| **LPUSH** | `LPUSH key value` | `LPUSH queue "task1"` | `1` (new length) | ✅ |   |
| **RPUSH** | `RPUSH key value` | `RPUSH queue "task2"` | `2` (new length) | ✅ |   |
| **LPOP**  | `LPOP key` | `LPOP queue` | `"task1"` | ✅ |   |
| **RPOP**  | `RPOP key` | `RPOP queue` | `"task2"` | ✅ |   |


#### Hashes (For Storing Objects Efficiently)

| Command  | Syntax | Example | Output | To implement | Done |
|----------|--------|---------|--------|-------------|------|
| **HSET**  | `HSET key field value` | `HSET user:1 name "Alice"` | `1` | ✅ |   |
| **HGET**  | `HGET key field` | `HGET user:1 name` | `"Alice"` | ✅ |   |
| **HDEL**  | `HDEL key field` | `HDEL user:1 name` | `1` | ✅ |   |
| **HGETALL** | `HGETALL key` | `HGETALL user:1` | `["name", "Alice"]` | ✅ |   |


#### Sets (For Unique Items, Tags, Sessions)

| Command  | Syntax | Example | Output | To implement | Done |
|----------|--------|---------|--------|-------------|------|
| **SADD**  | `SADD key value` | `SADD online_users "user1"` | `1` | ✅ |   |
| **SREM**  | `SREM key value` | `SREM online_users "user1"` | `1` | ✅ |   |
| **SMEMBERS** | `SMEMBERS key` | `SMEMBERS online_users` | `["user2", "user3"]` | ✅ |   |


#### Miscellaneous (For Admin, Debugging)Miscellaneous (For Admin, Debugging)

| Command  | Syntax | Example | Output | To implement | Done |
|----------|--------|---------|--------|-------------|------|
| **FLUSHDB** | `FLUSHDB` | `FLUSHDB` | `OK` | | ✅ |
| **FLUSHALL** | `FLUSHALL` | `FLUSHALL` | `OK` | ✅ |   |
| **KEYS** | `KEYS pattern` | `KEYS user:*` | `["user:1", "user:2"]` |  | ✅ |
