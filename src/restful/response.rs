use serde::{Serialize, Deserialize};
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct Node {
    #[prost(string, tag = "1")]
    pub node_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub neighbor_nodes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "3")]
    pub is_alive: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct NodesOverviewResponse {
    #[prost(message, repeated, tag = "1")]
    pub nodes: ::prost::alloc::vec::Vec<crate::response::Node>,
    #[prost(uint32, tag = "2")]
    pub total_node_count: u32,
    #[prost(uint32, tag = "3")]
    pub total_message_count: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct NodeId {
    #[prost(string, tag = "1")]
    pub node_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct MessageInfo {
    #[prost(string, tag = "1")]
    pub message_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub from_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub to_addr: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct NodeDetailResponse {
    #[prost(string, tag = "1")]
    pub node_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_alive: bool,
    #[prost(map = "string, int32", tag = "3")]
    pub clock: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    #[prost(message, repeated, tag = "4")]
    pub message_list: ::prost::alloc::vec::Vec<crate::response::MessageInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct MessageId {
    #[prost(string, tag = "1")]
    pub message_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct MessageDetailResponse {
    #[prost(string, tag = "1")]
    pub message_id: ::prost::alloc::string::String,
    #[prost(map = "string, int32", tag = "2")]
    pub clock: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    #[prost(string, tag = "5")]
    pub from_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub to_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub raw_message: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub signature: ::prost::alloc::string::String,
}