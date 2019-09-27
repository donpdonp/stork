mod config;
mod socket;

fn main() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let config_json = config::read("config.json").expect("config bad");
    let config = config::new(&config_json);
    println!("{:?}", config);
    socket::open(config.host).expect("no sock");
}
