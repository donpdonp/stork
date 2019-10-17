use std::sync::Arc;

use grpcio::{Channel, ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder};
use protobuf::Message;

use crate::config::Config;
use crate::config::Satellite;
use crate::protos::contact::{CheckInRequest, CheckInResponse};
use crate::protos::contact_grpc::NodeClient;
use crate::protos::node::NodeVersion;

pub fn grpc_connect(satellite: &Satellite, client_cert: &str, client_key: &str) -> Channel {
    let env = Arc::new(EnvBuilder::new().build());
    let cert_builder = ChannelCredentialsBuilder::new().cert(
        client_cert.as_bytes().to_vec(),
        client_key.as_bytes().to_vec(),
    );
    let cert = cert_builder.build();
    ChannelBuilder::new(env).secure_connect(satellite.ip, cert)
}

pub fn handshake(ch: Channel, config: Config) -> Result<CheckInResponse, grpcio::Error> {
    let nc = NodeClient::new(ch);
    let mut ver = NodeVersion::new();
    ver.set_version("0.22.1".to_string());
    let mut cir = CheckInRequest::default();
    // storagenode.go
    // conn.NodeClient().CheckIn(ctx, &pb.CheckInRequest{
    //             Address:  self.Address.GetAddress(),
    //             Version:  &self.Version,
    //             Capacity: &self.Capacity,
    //             Operator: &self.Operator,
    //         })
    cir.set_address("1.2.3.4".to_string());
    cir.set_version(ver);
    println!("Node check-in request: {:?}", cir);
    nc.check_in(&cir)
}
