use std::fs;
use serde_json::{Result, Value};

fn main() {
    println!("Hello, world!");
    let config: Value = read_config("config.json").expect("config bad");
    println!("config ip {}", config["ip"]);
}

fn read_config(filename: &str) -> Result<Value> {
    println!("reading {}", filename);
    let json = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    return serde_json::from_str(&json);
}
