mod config;
mod protos;
mod sjproto;
mod socket;

fn main() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let config_json = config::read("config.json").expect("config bad");
    let config = config::new(&config_json);
    println!("{:?}", config);

    let channel = sjproto::grpc_connect(
        &config.satellites[0],
        config.read_client_cert().as_str(),
        config.read_client_key().as_str(),
    );
    println!("{:?} connected", "channel");

    let reply_checkin = sjproto::handshake(channel, config);
    println!("Node check-in response: {:?} X", reply_checkin);
    let reply_stat = sjproto::stat(channel, config);
    //println!("Stat response: {:?} X", reply_stat);
}
