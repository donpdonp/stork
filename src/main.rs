use protos::contact::CheckInRequest;
use protos::contact_grpc::NodeClient;

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
        config.satellites[0].ip,
        config.read_client_cert().as_str(),
        config.read_client_key().as_str(),
    );
    println!("{:?} connected", config.bootstrap);

    let nc = NodeClient::new(ch);
    let ctr = CheckInRequest::default();
    //println!("Node check-in request: {:?} X", ctr);
    let reply = nc.check_in(&ctr);
    //sjproto::handshake(socket);
    println!("Node check-in response: {:?} X", reply);
}
