pub mod common {
    tonic::include_proto!("common");
    tonic::include_proto!("authorization");
    tonic::include_proto!("trading");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("moss-street");
}
