fn main() {
    // protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
    //     out_dir: "src/protos",
    //     input: &["protos/contact.proto", "protos/node.proto"],
    //     includes: &["protos"],
    //     customize: protobuf_codegen_pure::Customize {
    //         ..Default::default()
    //     },
    // })
    // .expect("protoc");

    let proto_root = "src/protos";
    println!("cargo:rerun-if-changed={}", proto_root);
    protoc_grpcio::compile_grpc_protos(
        &["protos/contact.proto", "protos/node.proto"],
        &["protos"],
        &proto_root,
        None,
    )
    .expect("Failed to compile gRPC definitions!");
}
