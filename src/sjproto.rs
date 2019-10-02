use protobuf::{CodedOutputStream, Message};
use std::io::Read;
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use tls_api::TlsConnector;
use tls_api::TlsConnectorBuilder;
use native_tls::TlsConnectorBuilder as MetalBuilder;
use crate::protos::{contact, node};

fn test_tls_connector() -> tls_api_native_tls::TlsConnector {
    let mut builder = tls_api_native_tls::TlsConnector::builder().unwrap();
    let mbu: &mut MetalBuilder = builder.underlying_mut();
    //mbu.danger_accept_invalid_certs(true);
    builder.build().unwrap()
}

pub fn grpc_connect(host: &str) -> Arc<grpc::Client> {
    let client_conf = Default::default();
    let mut tls_option =
        httpbis::ClientTlsOption::Tls("foobar.com".to_owned(), Arc::new(test_tls_connector()));
    let addr = SocketAddr::new(host.parse().unwrap(), 7777);
    let grpc_client =
        Arc::new(grpc::Client::new_expl(&addr, "foobar.com", tls_option, client_conf).unwrap());
    grpc_client
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
