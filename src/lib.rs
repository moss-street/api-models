pub mod common {
    tonic::include_proto!("auth.v1");
    tonic::include_proto!("trade.v1");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("moss-street");
}
