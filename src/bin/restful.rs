use std::collections::HashMap;
use gateway::db::get_conn;
use gateway::entities::node_info;
use gateway::restful::response::{MessageDetailResponse, MessageInfo, Node, NodeDetailResponse, NodesOverviewResponse};
use gateway::entities::{prelude::*, *};
use axum::{
    response::Html,
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use axum::handler::Handler;
use axum::extract::Path;
use http::HeaderValue;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, Any, CorsLayer};
use serde_json::Value;
use sea_orm::*;

async fn get_nodes_info() -> Result<Json<NodesOverviewResponse>, StatusCode> {
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
    Ok(Json(res))
}

async fn get_node_by_id(Path(id): Path<String>) -> Result<Json<NodeDetailResponse>, StatusCode>{
    let mut message_list: Vec<MessageInfo> = vec![];

    let conn = get_conn().await;

    let node: node_info::Model;
    let query_res = NodeInfo::find_by_id(id).one(conn).await.expect("REASON");
    match query_res {
        Some(query_res) => {
            node = query_res;
        }
        None => todo!()
    }

    let messages_query: Vec<clock_message::Model> = ClockMessage::find().filter(clock_message::Column::NodeId.eq(&node.node_id)).all(conn).await.expect("REASON");
    let mut max_clock = 0;
    for m in &messages_query {
        let clock: HashMap<String, Value> = m.clock.as_object().unwrap().iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        // println!("{}", clock.values().next().unwrap());
        let str = clock.values().next().unwrap().to_string();
        let num: i32 = str.parse().unwrap();
        if num > max_clock {
            max_clock = num
        }
        message_list.push(MessageInfo {
            message_id: m.message_id.clone(),
            from_addr: m.from_addr.clone().unwrap_or_default(),
            to_addr: m.to_addr.clone().unwrap_or_default(),
        })
    }
    let mut clock: HashMap<String, i32> = HashMap::new();
    clock.insert(node.node_id.clone(), max_clock);

    let res = NodeDetailResponse {
        node_id: node.node_id.clone(),
        is_alive: node.is_alive.clone(),
        clock,
        message_list,
    };
    Ok(Json(res))
}


async fn get_message_by_id(Path(id): Path<String>) -> Result<Json<MessageDetailResponse>, StatusCode> {
    let conn = get_conn().await;

    let message: clock_message::Model;
    let query_res = ClockMessage::find_by_id(id).one(conn).await.expect("REASON");
    match query_res {
        Some(query_res) => {
            message = query_res;
        }
        None => todo!()
    }

    let clock_obj: HashMap<String, Value> = message.clock.as_object().unwrap().iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    let node_id = clock_obj.keys().next().unwrap().to_string();
    let count = clock_obj.values().next().unwrap().to_string();
    let count: i32 = count.parse().unwrap();
    let mut clock: HashMap<String, i32> = HashMap::new();
    clock.insert(node_id, count);

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
    Ok(Json(res))
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers([http::header::AUTHORIZATION]);

    let app = Router::new()
        .nest(
            "/gateway",
            Router::new()
                .route("/overview", get(get_nodes_info))
                .route("/node/:id", get(get_node_by_id))
                .route("/message/:id", get(get_message_by_id)).layer(cors)
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
