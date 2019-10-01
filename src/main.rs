use grpcio::{ChannelBuilder, EnvBuilder};
use std::sync::Arc;

mod config;
mod socket;
mod sjproto;
mod protos {
    pub mod contact;
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
    //sjproto::handshake(socket);
}
