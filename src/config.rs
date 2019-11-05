use serde_json::{Result, Value};
use std::fs;

#[derive(Debug)]
pub struct Config<'a> {
    pub myip: &'a str,
    pub email: &'a str,
    pub wallet: &'a str,
    pub satellites: Vec<Satellite<'a>>,
    pub storj: Storj<'a>,
}

#[derive(Debug)]
pub struct Storj<'a> {
    pub config: &'a str,
    pub commit_hash: &'a str,
    pub version: &'a str,
}

#[derive(Debug)]
pub struct Satellite<'a> {
    pub id: &'a str,
    pub ip: &'a str,
}

pub fn new<'a>(config_file: &'a Value) -> Config<'a> {
    let mut config = Config {
        myip: config_file["myip"].as_str().unwrap(),
        email: config_file["email"].as_str().unwrap(),
        wallet: config_file["wallet"].as_str().unwrap(),
        satellites: Vec::new(),
        storj: Storj {
            config: config_file["storj"]["config"].as_str().unwrap(),
            version: config_file["storj"]["version"].as_str().unwrap(),
            commit_hash: config_file["storj"]["commit_hash"].as_str().unwrap(),
        },
    };
    for jsat in config_file["satellites"].as_array().unwrap() {
        println!("{:?}\n", jsat);
        config.satellites.push(Satellite {
            id: jsat["id"].as_str().unwrap(),
            ip: jsat["ip"].as_str().unwrap(),
        });
    }
    config
}

pub fn read(filename: &str) -> Result<Value> {
    let json = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return serde_json::from_str(&json);
}

impl Config<'_> {
    pub fn read_client_cert(&self) -> String {
        let path = self.storj.config.to_string() + "/identity/storagenode/identity.cert";
        return fs::read_to_string(path).expect("bad client cert");
    }

    pub fn read_client_key(&self) -> String {
        let path = self.storj.config.to_string() + "/identity/storagenode/identity.key";
        return fs::read_to_string(path).expect("bad client key");
    }
}
