#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
// False positive with None
#![allow(non_snake_case)]

pub mod aof;
pub mod commands;
pub mod config;
pub mod database;
pub mod keys;
pub mod operation;
pub mod parser;
pub mod process;
pub mod server;
pub mod types;
