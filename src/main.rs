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
    let socket = &mut socket::open(config.bootstrap).expect("no sock");
    sjproto::handshake(socket);
}
