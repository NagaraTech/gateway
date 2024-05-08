use futures::Stream;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use sea_orm::DatabaseConnection;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status};
use crate::grpc::gateway::gateway_server::Gateway;
use crate::grpc::gateway::{NodesOverviewResponse, NodeDetailResponse, MessageDetailResponse, NodeId, MessageId, Node, MessageInfo};
use crate::entities::{prelude::*, *};
use sea_orm::*;
use crate::db::get_conn;
use serde_json::{Map, Value};

const EMPTY_NODEID_ERR: &str = "provided node is  was empty";
const NODE_NOT_EXIST_ERR: &str = "node does not exist";

#[derive(Debug)]
pub struct GatewayResponse {}

#[tonic::async_trait]
impl Gateway for GatewayResponse {
    async fn node_list(&self, request: Request<()>) -> Result<Response<NodesOverviewResponse>, Status> {
        let conn = get_conn().await;
        let nodes: Vec<node_info::Model> = NodeInfo::find().all(conn).await.expect("REASON");
        let message_count = ClockMessage::find().count(conn).await.expect("REASON");
        let mut nodes_response: Vec<Node> = vec![];
        for n in &nodes {
            nodes_response.push(Node {
                node_id: n.node_id.clone(),
                neighbor_nodes: n.neighbor_nodes.clone(),
                is_alive: n.is_alive.clone(),
            })
        }
        let res = NodesOverviewResponse {
            nodes: nodes_response,
            total_node_count: nodes.len() as u32,
            total_message_count: message_count as u32,
        };
        Ok(Response::new(res))
    }

    async fn node_detail(&self, request: Request<NodeId>) -> Result<Response<NodeDetailResponse>, Status> {
        let identifier = request.into_inner();

        if identifier.node_id == "" {
            return Err(Status::invalid_argument(EMPTY_NODEID_ERR));
        }

        let mut message_list: Vec<MessageInfo> = vec![];

        let conn = get_conn().await;

        let node: node_info::Model;
        let query_res = NodeInfo::find_by_id(identifier.node_id).one(conn).await.expect("REASON");
        match query_res {
            Some(query_res) => {
                node = query_res;
            }
            None => return Err(Status::invalid_argument(NODE_NOT_EXIST_ERR))
        }

        let messages_query: Vec<clock_message::Model> = ClockMessage::find().filter(clock_message::Column::NodeId.eq(&node.node_id)).all(conn).await.expect("REASON");
        let mut max_clock = 0;
        for m in &messages_query {
            let clock: HashMap<String, Value> = m.clock.as_object().unwrap().iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            // println!("{}", clock.values().next().unwrap());
            let str = clock.values().next().unwrap().to_string();
            let num: i32 = str.parse().unwrap();
            if num > max_clock{
                max_clock = num
            }
            message_list.push(MessageInfo {
                message_id: m.message_id.clone(),
                from_addr: m.from_addr.clone().unwrap_or_default(),
                to_addr: m.to_addr.clone().unwrap_or_default(),
            })
        }
        let mut clock: HashMap<String, i32> = HashMap::new();
        clock.insert(node.node_id.clone(),max_clock);

        let res = NodeDetailResponse {
            node_id: node.node_id.clone(),
            is_alive: node.is_alive.clone(),
            clock,
            message_list,
        };

    Ok(Response::new(res))
}

async fn message_detail(&self, request: Request<MessageId>) -> Result<Response<MessageDetailResponse>, Status> {
    let identifier = request.into_inner();

    if identifier.message_id == "" {
        return Err(Status::invalid_argument(EMPTY_NODEID_ERR));
    }

    let conn = get_conn().await;

    let message: clock_message::Model;
    let query_res = ClockMessage::find_by_id(identifier.message_id).one(conn).await.expect("REASON");
    match query_res {
        Some(query_res) => {
            message = query_res;
        }
        None => return Err(Status::invalid_argument(NODE_NOT_EXIST_ERR))
    }

    let clock_obj: HashMap<String, Value> = message.clock.as_object().unwrap().iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    let node_id = clock_obj.keys().next().unwrap().to_string();
    let count = clock_obj.values().next().unwrap().to_string();
    let count: i32 = count.parse().unwrap();
    let mut clock: HashMap<String, i32> = HashMap::new();
    clock.insert(node_id,count);

    let res = MessageDetailResponse {
        message_id: message.message_id.clone(),
        clock,
        event_count: message.event_count.clone(),
        is_zk: message.is_zk.clone(),
        from_addr: message.from_addr.unwrap().clone(),
        to_addr: message.to_addr.unwrap().clone(),
        raw_message: message.raw_message.unwrap().clone(),
        signature: message.signature.clone(),
    };
    Ok(Response::new(res))
}
}