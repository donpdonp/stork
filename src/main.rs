mod socket;
mod config;

fn main() {
    println!("Hello, world!");
    let config_json = config::read("config.json").expect("config bad");
    let mut config = config::new(&config_json);
    println!("config {:?}", config);
    socket::open(config.host).expect("no sock");
    config.host = "else";
}
