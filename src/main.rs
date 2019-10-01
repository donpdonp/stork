//use futures::prelude::*;
use grpcio::{ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder};
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
    let config = config::new(&config_json);
    println!("{:?}", config);

    let ch = sjproto::grpc_connect(config.bootstrap);
    println!("{:?} connected", config.bootstrap);

    let nc = NodeClient::new(ch);
    let ctr = CheckInRequest::default();
    println!("Node check-in request: {:?} X", ctr);
    let reply = nc.check_in(&ctr); //.expect("rpc");
    //sjproto::handshake(socket);
    println!("Node check-in response: {:?} X", reply);
}
