use std::fs;
use protos::contact::CheckInRequest;
use protos::contact_grpc::{NodeClient};

mod config;
mod sjproto;
mod socket;
mod protos {
    pub mod contact;
    pub mod contact_grpc;
    pub mod node;
}

fn main() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let config_json = config::read("config.json").expect("config bad");
    let client_cert = fs::read_to_string("client.cert").expect("bad client cert");
    let client_key = fs::read_to_string("client.key").expect("bad client key");
    let config = config::new(&config_json, &client_cert, &client_key);
    println!("{:?}", config);

    let ch = sjproto::grpc_connect(config.bootstrap, config.client_cert, config.client_key);
    println!("{:?} connected", config.bootstrap);

    let nc = NodeClient::new(ch);
    let ctr = CheckInRequest::default();
    //println!("Node check-in request: {:?} X", ctr);
    let reply = nc.check_in(&ctr);
    //sjproto::handshake(socket);
    println!("Node check-in response: {:?} X", reply);
}
