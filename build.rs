fn main() {
    // protobuf-codegen-pure = "2.8.1"
    // protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
    //     out_dir: "src/protos",
    //     input: &["protos/contact.proto", "protos/node.proto"],
    //     includes: &["protos"],
    //     customize: protobuf_codegen_pure::Customize {
    //         ..Default::default()
    //     },
    // })
    // .expect("protoc");

    // protoc-grpcio = "1.1.0"
    let proto_root = "src/protos";
    println!("cargo:rerun-if-changed={}", proto_root);
    protoc_grpcio::compile_grpc_protos(
        &["protos/contact.proto", "protos/node.proto"],
        &["protos"],
        &proto_root,
        None,
    )
    .expect("Failed to compile gRPC definitions!");

    // protoc_rust_grpc::run(protoc_rust_grpc::Args {
    //     out_dir: "src/protos",
    //     includes: &["protos"],
    //     input: &["protos/contact.proto", "protos/node.proto"],
    //     rust_protobuf: true, // also generate protobuf messages, not just services
    //     ..Default::default()
    // }).expect("protoc-rust-grpc");
}
