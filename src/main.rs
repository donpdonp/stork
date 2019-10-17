mod config;
mod protos;
mod sjproto;
mod socket;

fn main() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let config_json = config::read("config.json").expect("config bad");
    let config = config::new(&config_json);
    println!("{:?}", config);

    let ch = sjproto::grpc_connect(
        &config.satellites[0],
        config.read_client_cert().as_str(),
        config.read_client_key().as_str(),
    );
    println!("{:?} connected", "ch here");

    let reply = sjproto::handshake(ch, config);
    println!("Node check-in response: {:?} X", reply);
}
