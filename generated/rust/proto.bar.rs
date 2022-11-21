// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoBarRequest {
    #[prost(int64, tag="1")]
    pub id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoBarResponse {
    #[prost(int64, tag="1")]
    pub id: i64,
}
include!("proto.bar.tonic.rs");
// @@protoc_insertion_point(module)
