use serde_json::{Result, Value};
use std::fs;

#[derive(Debug)]
pub struct Config<'a> {
    pub myip: &'a str,
    pub email: &'a str,
    pub wallet: &'a str,
    pub satellites: [Satellite<'a>; 1],
    pub storj_config: &'a str,
}

#[derive(Debug)]
pub struct Satellite<'a> {
    pub id: &'a str,
    pub ip: &'a str,
}

pub fn new<'a>(config_file: &'a Value) -> Config<'a> {
    let onesat = &config_file["satellites"][0];
    Config {
        myip: config_file["myip"].as_str().unwrap(),
        email: config_file["email"].as_str().unwrap(),
        wallet: config_file["wallet"].as_str().unwrap(),
        satellites: [Satellite {
            id: onesat["id"].as_str().unwrap(),
            ip: onesat["ip"].as_str().unwrap(),
        }],
        storj_config: config_file["storj_config"].as_str().unwrap(),
    }
}

pub fn read(filename: &str) -> Result<Value> {
    let json = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return serde_json::from_str(&json);
}

impl Config<'_> {
    pub fn read_client_cert(&self) -> String {
        let path = self.storj_config.to_string() + "/identity/storagenode/identity.cert";
        return fs::read_to_string(path).expect("bad client cert");
    }

    pub fn read_client_key(&self) -> String {
        let path = self.storj_config.to_string() + "/identity/storagenode/identity.key";
        return fs::read_to_string(path).expect("bad client key");
    }
}
