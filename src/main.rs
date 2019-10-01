use grpcio::{ChannelBuilder, EnvBuilder};
use std::sync::Arc;
use futures::prelude::*;
use protos::contact_grpc::ContactClient;

mod config;
mod socket;
mod sjproto;
mod protos {
    pub mod contact;
    pub mod contact_grpc;
    pub mod node;
}

fn main() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let config_json = config::read("config.json").expect("config bad");
    let config = config::new(&config_json);
    println!("{:?}", config);

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(config.bootstrap);
    println!("{:?} connected", config.bootstrap);
    let ct = ContactClient::new(ch);
    //sjproto::handshake(socket);
}
