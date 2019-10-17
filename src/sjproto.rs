use grpcio::{Channel, ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder};
use protobuf::{CodedOutputStream, Message};
use std::io::Read;
use std::net::TcpStream;

use crate::protos::{contact, node};
use std::sync::Arc;

pub fn grpc_connect(host: &str, client_cert: &str, client_key: &str) -> Channel {
    let env = Arc::new(EnvBuilder::new().build());
    let cert_builder = ChannelCredentialsBuilder::new().cert(
        client_cert.as_bytes().to_vec(),
        client_key.as_bytes().to_vec(),
    );
    let cert = cert_builder.build();
    ChannelBuilder::new(env).secure_connect(host, cert)
}

pub fn handshake(stream: &mut TcpStream) {
    let _timestamp = ::protobuf::well_known_types::Timestamp::new();
    let _nodeVersion = node::NodeVersion::new();
    //  {
    //     version: "1".to_string(),
    //     commit_hash: "ab".to_string(),
    //     timestamp: protobuf::SingularPtrField::some(timestamp),

    // };
    let creq = contact::CheckInRequest::new();
    //  {
    //     address: "1.2".to_string(),
    //     cached_size: ::protobuf::CachedSize {
    //         size: AtomicUsize::new(42),
    //     },
    //     version: protobuf::SingularPtrField::some(nodeVersion),
    // };
    let mut cos = CodedOutputStream::new(stream);
    creq.write_to(&mut cos);

    stream.read(&mut [0; 128]); // wait
}
