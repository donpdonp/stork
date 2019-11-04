extern crate clap;
use clap::{App, SubCommand};

mod config;
mod protos;
mod sjproto;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                    .version(env!("CARGO_PKG_VERSION"))
                    .subcommand(SubCommand::with_name("rep"))
                    .get_matches();
    let config_file = matches.value_of("config").unwrap_or("config.json");
    println!("Value for config: {}", config_file);

    let config_json = config::read(config_file).expect("config bad");
    let config = config::new(&config_json);
    println!("{:?}", config);

    if let Some(matches) = matches.subcommand_matches("rep") {
        let channel2 = sjproto::grpc_connect(
            &config.satellites[0],
            config.read_client_cert().as_str(),
            config.read_client_key().as_str(),
        );
        println!("connected.");
        let reply_stat = sjproto::stat(channel2);
        println!("Stat response: {:?} X", reply_stat);
    }

    // let channel = sjproto::grpc_connect(
    //     &config.satellites[0],
    //     config.read_client_cert().as_str(),
    //     config.read_client_key().as_str(),
    // );
    // println!("{:?} connected", "channel");

    // let reply_checkin = sjproto::handshake(channel, &config);
    // println!("Node check-in response: {:?} X", reply_checkin);

}
