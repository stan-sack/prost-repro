// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoFooRequest {
    #[prost(int64, tag="1")]
    pub id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoFooResponse {
    #[prost(int64, tag="1")]
    pub id: i64,
}
include!("proto.foo.tonic.rs");
// @@protoc_insertion_point(module)
