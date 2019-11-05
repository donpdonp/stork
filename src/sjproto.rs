use std::sync::Arc;

use grpcio::{Channel, ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder};
use protobuf::well_known_types::Timestamp;

use crate::config::Config;
use crate::config::Satellite;
use crate::protos::contact::{CheckInRequest, CheckInResponse};
use crate::protos::contact_grpc::NodeClient;
use crate::protos::node::{NodeCapacity, NodeOperator, NodeVersion};
use crate::protos::nodestats::{GetStatsRequest, GetStatsResponse};
use crate::protos::nodestats_grpc::NodeStatsClient;

pub fn grpc_connect(satellite: &Satellite, client_cert: &str, client_key: &str) -> Channel {
    let channel_cred_builder = ChannelCredentialsBuilder::new().cert(
        client_cert.as_bytes().to_vec(),
        client_key.as_bytes().to_vec(),
    );
    let channel_cred = channel_cred_builder.build();
    let env = Arc::new(EnvBuilder::new().build());
    let chan_builder = ChannelBuilder::new(env);
    chan_builder.secure_connect(satellite.ip, channel_cred)
}

pub fn handshake(ch: Channel, config: &Config) -> Result<CheckInResponse, grpcio::Error> {
    let nc = NodeClient::new(ch);
    let mut ver = NodeVersion::new();
    ver.set_commit_hash(config.storj.commit_hash.to_string());
    let mut ts = Timestamp::new();
    ts.set_seconds(1572640075);
    ver.set_timestamp(ts);
    ver.set_release(true);
    ver.set_version(config.storj.version.to_string());
    let mut check_in_request = CheckInRequest::default();
    // storagenode.go
    // conn.NodeClient().CheckIn(ctx, &pb.CheckInRequest{
    //             Address:  self.Address.GetAddress(),
    //             Version:  &self.Version,
    //             Capacity: &self.Capacity,
    //             Operator: &self.Operator,
    //         })
    check_in_request.set_address(config.myip.to_string());
    check_in_request.set_version(ver);
    let mut capacity = NodeCapacity::new();
    capacity.set_free_bandwidth(1000000000);
    capacity.set_free_disk(1000000000);
    check_in_request.set_capacity(capacity);
    let mut op = NodeOperator::new();
    op.set_email(config.email.to_string());
    op.set_wallet(config.wallet.to_string());
    check_in_request.set_operator(op);
    println!("Node check-in request: {:?}", check_in_request);
    nc.check_in(&check_in_request)
}

pub fn stat(ch: Channel) -> Result<GetStatsResponse, grpcio::Error> {
    let nsc = NodeStatsClient::new(ch);
    let nsr = GetStatsRequest::new();
    nsc.get_stats(&nsr)
}
