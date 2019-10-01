use protobuf::{CodedOutputStream, Message};
use std::io::Read;
use std::net::TcpStream;
use std::sync::Arc;
use std::fs::File;
use grpcio::{Channel, ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder};

use crate::protos::{contact, node};

pub fn grpc_connect(host: &str) -> Channel {
    let env = Arc::new(EnvBuilder::new().build());
    // let mut f = File::open("foo.txt")?;
    // let mut key_bytes = Vec::new();
    // f.read_to_end(&mut key_bytes)?;
    // let mut f = File::open("foo.txt")?;
    // let mut cert_bytes = Vec::new();
    // f.read_to_end(&mut cert_bytes)?;
    let rootcert = "".as_bytes().to_vec();
    let cert = ChannelCredentialsBuilder::new().root_cert(rootcert).build();
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
