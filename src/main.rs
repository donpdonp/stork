extern crate clap;
use clap::{App, Arg, SubCommand};

mod config;
mod protos;
mod sjproto;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const COMMANDS: [&'static str; 2] = ["reputation", "checkin"];
enum Commands {
    Reputation,
    CheckIn,
    NoCommand,
}

fn main() {
    let options = cli();

    let config_file = options.value_of("config").unwrap_or("config.json");
    let config_json = config::read(config_file).expect("config bad");
    let config = config::new(&config_json);
    println!("{:?} loaded", config.myip);

    match subcommand(options) {
        Commands::Reputation => do_reputation(&config),
        Commands::CheckIn => do_check_in(&config),
        Commands::NoCommand => (),
    }
}

fn do_reputation(config: &config::Config) {
    hello();
    for satellite in &config.satellites {
        println!("Connecting to {}", satellite.ip);
        let channel2 = sjproto::grpc_connect(
            satellite,
            config.read_client_cert().as_str(),
            config.read_client_key().as_str(),
        );
        println!("connected.");
        let reply_stat = sjproto::stat(channel2);
        println!("Stat response: {:?} X", reply_stat);
    }
}

fn do_check_in(config: &config::Config) {
    hello();
    let channel = sjproto::grpc_connect(
        &config.satellites[0],
        config.read_client_cert().as_str(),
        config.read_client_key().as_str(),
    );
    println!("{:?} connected", "channel");

    let reply_checkin = sjproto::handshake(channel, &config);
    println!("Node check-in response: {:?} X", reply_checkin);
}

fn hello() {
    println!("{} {}", NAME, VERSION);
}

fn cli() -> clap::ArgMatches<'static> {
    let mut clap_builder = App::new(NAME).version(VERSION).arg(
        Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("path to config file. default: config.json")
            .takes_value(true),
    );
    for subcommand_name in &COMMANDS {
        clap_builder = clap_builder.subcommand(SubCommand::with_name(subcommand_name))
    }
    clap_builder.get_matches()
}

fn subcommand(matches: clap::ArgMatches) -> Commands {
    if matches.subcommand_matches(COMMANDS[0]).is_some() {
        Commands::Reputation
    } else if matches.subcommand_matches(COMMANDS[1]).is_some() {
        Commands::CheckIn
    } else {
        Commands::NoCommand
    }
}
