use std::fs;
use serde_json::{Result, Value};

#[derive(Debug)]
pub struct Config<'a> {
    pub host: &'a str
}

pub fn new<'a>(config_file: &'a Value) -> Config<'a> {
    Config{ host: config_file["host"].as_str().unwrap() }
}

pub fn read(filename: &str) -> Result<Value> {
    println!("reading {}", filename);
    let json = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    return serde_json::from_str(&json);
}
