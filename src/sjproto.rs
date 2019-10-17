use grpcio::{Channel, ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder};
use protobuf::Message;
use crate::config::Satellite;

//use crate::protos::{contact, node};
use std::sync::Arc;

pub fn grpc_connect(satellite: &Satellite, client_cert: &str, client_key: &str) -> Channel {
    let env = Arc::new(EnvBuilder::new().build());
    let cert_builder = ChannelCredentialsBuilder::new().cert(
        client_cert.as_bytes().to_vec(),
        client_key.as_bytes().to_vec(),
    );
    let cert = cert_builder.build();
    ChannelBuilder::new(env).secure_connect(satellite.ip, cert)
}
