fn main() {
  protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
      out_dir: "src/protos",
      input: &["protos/contact.proto", "protos/node.proto"],
      includes: &["protos"],
      customize: protobuf_codegen_pure::Customize {
        ..Default::default()
      },
  }).expect("protoc");
}