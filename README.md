# 🐆 Sider

![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/bourdeau/sider) [![build status](https://github.com/bourdeau/sider/actions/workflows/build.yml/badge.svg)](https://github.com/bourdeau/sider/actions) [![dependency status](https://deps.rs/repo/github/bourdeau/sider/status.svg)](https://deps.rs/repo/github/bourdeau/sider)

Sider is a Redis-compatible server reimplemented from scratch in Rust. It aims to provide a in-memory key-value store while leveraging Rust’s safety and concurrency features. Currently, Sider is only supported on Linux, with potential cross-platform support planned for the future.

I make this project for fun, it doesn't aim to replace Redis. For now, I am focusing on implementing common Redis functionalities rather than on performance.

![Flamegraph](flamegraph.svg)

## Setup

Run the sider server:

```bash
cargo build --release
./target/release/sider
```
In another terminal:

```
redis-cli
SET first_name John
```

## Benchmark

On average, Sider is 40% slower than Redis, which came as a surprise, as I was expecting much worse performance considering I didn't make any optimizations.

On my machine:
```
OS: NixOS 24.11 (Vicuna)
KERNEL: 6.6.74 
CPU: AMD Ryzen 7 9700X 8-Core
RAM: 31 GB
```

*Redis 7.2.7:*

```
➜ redis-benchmark -t set,get,incr,lpush,rpush,lpop,rpop,hset,lpush,lrange_100,lrqnge_300,lrange_500,lrange_600 -n 100000 -q
SET: 306748.47 requests per second, p50=0.087 msec
GET: 367647.03 requests per second, p50=0.071 msec
INCR: 367647.03 requests per second, p50=0.071 msec
LPUSH: 369003.69 requests per second, p50=0.071 msec
RPUSH: 369003.69 requests per second, p50=0.071 msec
LPOP: 371747.22 requests per second, p50=0.071 msec
RPOP: 373134.31 requests per second, p50=0.071 msec
HSET: 371747.22 requests per second, p50=0.071 msec
LPUSH (needed to benchmark LRANGE): 374531.84 requests per second, p50=0.071 msec
LRANGE_100 (first 100 elements): 232018.56 requests per second, p50=0.111 msec
LRANGE_500 (first 500 elements): 69832.40 requests per second, p50=0.359 msec
LRANGE_600 (first 600 elements): 59594.76 requests per second, p50=0.415 msec
```

*Sider 0.6.0:*

```
➜ redis-benchmark -t set,get,incr,lpush,rpush,lpop,rpop,hset,lpush,lrange_100,lrqnge_300,lrange_500,lrange_600 -n 100000 -q
SET: 223713.64 requests per second, p50=0.175 msec
GET: 261096.61 requests per second, p50=0.119 msec
INCR: 245700.25 requests per second, p50=0.159 msec
LPUSH: 79681.27 requests per second, p50=0.623 msec
RPUSH: 229357.80 requests per second, p50=0.175 msec
LPOP: 30506.40 requests per second, p50=1.631 msec
RPOP: 222717.16 requests per second, p50=0.175 msec
HSET: 210526.31 requests per second, p50=0.183 msec
LPUSH (needed to benchmark LRANGE): 77881.62 requests per second, p50=0.647 msec
LRANGE_100 (first 100 elements): 156006.25 requests per second, p50=0.191 msec
LRANGE_500 (first 500 elements): 60901.34 requests per second, p50=0.447 msec
LRANGE_600 (first 600 elements): 52521.01 requests per second, p50=0.519 msec
```


## Features

`Sider` implements a basic `Append-Only File (AOF)` system. `AOF` persistence logs every write operation received by the server, allowing these operations to be replayed during startup to restore the original dataset.

### Commands

#### Key-Value Storage

| Command  | Syntax | Example | Output | Done |
|----------|--------|---------|--------|------|
| **SET**  | `SET key value` | `SET user:1 "John"` | `OK` | ✅ |
| **GET**  | `GET key` | `GET user:1` | `"John"` | ✅ |
| **DEL**  | `DEL key` | `DEL user:1` | `1` (if key existed) | ✅ |
| **EXISTS** | `EXISTS key` | `EXISTS user:1` | `1` (exists) / `0` (not) | ✅ |


#### Expiration & Time-to-Live

**Note:** expired keys are deleted by calling `GET` or by a background task every 60 seconds.

| Command  | Syntax | Example | Output | Done |
|----------|--------|---------|--------|------|
| **EXPIRE** | `EXPIRE key seconds` | `EXPIRE user:1 3600` | `1` (success) | ✅ |
| **TTL**  | `TTL key` | `TTL user:1` | `3599` (seconds left) | ✅ |


#### Counters & Rate Limiting

| Command  | Syntax | Example | Output | Done |
|----------|--------|---------|--------|------|
| **INCR**  | `INCR key` | `INCR api:requests` | `1`, `2`, `3`... | ✅ |
| **DECR**  | `DECR key` | `DECR api:requests` | `2`, `1`, `0`... | ✅ |
| **INCRBY** | `INCRBY key amount` | `INCRBY api:requests 5` | `5`, `10`, `15`... | ✅ |


#### Lists

| Command  | Syntax | Example | Output | Done |
|----------|--------|---------|--------|------|
| **LPUSH** | `LPUSH key values` | `LPUSH queue task1 task2 task3` | `1` (new length) | ✅ |
| **LRANGE** | `LRANGE key start stop` | `LRANGE queue 0 -1` | `2` (new length) | ✅ |
| **RPUSH** | `RPUSH key value` | `RPUSH queue "task2"` | `2` (new length) | ✅ |
| **LPOP**  | `LPOP key` | `LPOP queue` | `"task1"` | ✅ |
| **RPOP**  | `RPOP key` | `RPOP queue` | `"task2"` | ✅ |


#### Hashes

| Command  | Syntax | Example | Output | Done |
|----------|--------|---------|--------|------|
| **HSET**  | `HSET key field value` | `HSET user:1 name "Alice"` | `1` | ✅ |
| **HGET**  | `HGET key field` | `HGET user:1 name` | `"Alice"` | ✅ |
| **HDEL**  | `HDEL key field` | `HDEL user:1 name` | `1` | ✅ |
| **HGETALL** | `HGETALL key` | `HGETALL user:1` | `["name", "Alice"]` | ✅ |


#### Sets

| Command  | Syntax | Example | Output | Done |
|----------|--------|---------|--------|------|
| **SADD**  | `SADD key value` | `SADD online_users "user1"` | `1` | ✅ |
| **SREM**  | `SREM key value` | `SREM online_users "user1"` | `1` |   |
| **SMEMBERS** | `SMEMBERS key` | `SMEMBERS online_users` | `["user2", "user3"]` | ✅ |


#### Miscellaneous

| Command  | Syntax | Example | Output | Done |
|----------|--------|---------|--------|------|
| **FLUSHDB** | `FLUSHDB` | `FLUSHDB` | `OK` | ✅ |
| **FLUSHALL** | `FLUSHALL` | `FLUSHALL` | `OK` |   |
| **KEYS** | `KEYS pattern` | `KEYS user:*` | `["user:1", "user:2"]` | ✅ |
