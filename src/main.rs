
mod socket;
mod config;

fn main() {
    println!("Hello, world!");
    let config_json = config::read("config.json").expect("config bad");
    let mut config = config::new(&config_json);
    println!("config {:?}", config);
    //socket::open(config.host).expect("no sock");

  // println!("pb codegen start");
  // protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
  //     out_dir: "src/protos",
  //     input: &["protos/contact.proto"],
  //     includes: &["protos"],
  //     customize: protobuf_codegen_pure::Customize {
  //       ..Default::default()
  //     },
  // }).expect("protoc");
  // println!("pb codegen stop");
}