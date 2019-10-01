use std::net::TcpStream;
use std::io::Read;
use protobuf::{CodedInputStream, CodedOutputStream, Message};

use crate::protos::{contact, node};

pub fn handshake(stream: &mut TcpStream) {
    let timestamp = ::protobuf::well_known_types::Timestamp::new();
    let nodeVersion = node::NodeVersion::new();
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
