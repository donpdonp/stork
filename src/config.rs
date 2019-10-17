use serde_json::{Result, Value};
use std::fs;

#[derive(Debug)]
pub struct Config<'a> {
    pub ip: &'a str,
    pub bootstrap: &'a str,
    pub satellites: [&'a str; 1],
    pub storj_id: &'a str,
    pub client_cert: &'a str,
    pub client_key: &'a str,
}

pub struct Satellite<'a> {
    pub id: &'a str,
    pub ip: &'a str,
}

pub fn new<'a>(config_file: &'a Value, client_cert: &'a str, client_key: &'a str) -> Config<'a> {
    Config {
        ip: config_file["ip"].as_str().unwrap(),
        bootstrap: config_file["bootstrap"].as_str().unwrap(),
        satellites: [config_file["satellites"][0].as_str().unwrap()],
        storj_id: config_file["storj_id"].as_str().unwrap(),
        client_cert: client_cert,
        client_key: client_key,
    }
}

pub fn read(filename: &str) -> Result<Value> {
    println!("reading {}", filename);
    let json = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return serde_json::from_str(&json);
}

// impl Config<'a> {
//     pub fn go() { }
// }