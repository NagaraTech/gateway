use std::collections::HashMap;
use gateway::db::get_conn;
// use gateway::entities::node_info;
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
use sea_orm::{Database, DatabaseConnection, DbErr, EntityTrait, QuerySelect, TryFromU64};
use sea_orm::entity::*;
use sea_orm::query::*;
use gateway::nodes::node::P2PNode;
use gateway::entities::{z_messages, merge_logs, clock_infos, node_info};
use tokio::task;
use tokio::time::{self, Duration};
use std::sync::{Arc, Mutex};

async fn get_nodes_info() -> Result<Json<NodesOverviewResponse>, StatusCode> {
    let conn = get_conn().await;
    let nodes: Vec<node_info::Model> = NodeInfo::find().all(conn).await.expect("REASON");
    let message_count = z_messages::Entity::find().count(conn).await.expect("REASON");
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

async fn get_node_by_id(Path(id): Path<String>) -> Result<Json<NodeDetailResponse>, StatusCode> {
    let mut message_list: Vec<MessageInfo> = vec![];
    // let id: i32 = id.parse().expect("Failed to convert string to i32");
    let conn = get_conn().await;

    let node_info_query = node_info::Entity::find().filter(node_info::Column::NodeId.eq(id.clone())).one(conn).await.expect("Fail to query").unwrap();
    let messages_query: Vec<z_messages::Model> = z_messages::Entity::find().filter(z_messages::Column::NodeId.eq(id.clone())).all(conn).await.expect("Fail to query");
    let node_clock_infos_query: Vec<clock_infos::Model> = clock_infos::Entity::find().filter(clock_infos::Column::NodeId.eq(id.clone())).all(conn).await.expect("Fail to query");

    let mut max_clock = 0;
    for clock_info in node_clock_infos_query {
        let clock: HashMap<String, u32> = serde_json::from_str(&*clock_info.clock).expect("JSON was not well-formatted");
        let str = clock.values().next().unwrap().to_string();
        let num: i32 = str.parse().unwrap();
        if num > max_clock {
            max_clock = num
        }
    }

    for m in &messages_query {
        message_list.push(MessageInfo {
            message_id: m.message_id.clone(),
            from_addr: m.from.clone(),
            to_addr: m.to.clone(),
        })
    }
    let mut clock: HashMap<String, i32> = HashMap::new();
    clock.insert(node_info_query.node_id.clone(), max_clock);

    let res = NodeDetailResponse {
        node_id: id.clone(),
        is_alive: node_info_query.is_alive.clone(),
        clock,
        message_list,
    };
    Ok(Json(res))
}

async fn get_message_by_id(Path(id): Path<String>) -> Result<Json<MessageDetailResponse>, StatusCode> {
    let conn = get_conn().await;

    let message: z_messages::Model;
    let query_res = z_messages::Entity::find().filter(z_messages::Column::MessageId.eq(id.clone())).one(conn).await.expect("REASON");
    match query_res {
        Some(query_res) => {
            message = query_res;
        }
        None => return Err(StatusCode::NOT_FOUND)
    }

    let node_clock_infos_query: Vec<clock_infos::Model> = clock_infos::Entity::find().filter(clock_infos::Column::NodeId.eq(id.clone())).all(conn).await.expect("Fail to query");

    let mut max_clock = 0;
    for clock_info in node_clock_infos_query {
        let clock: HashMap<String, u32> = serde_json::from_str(&*clock_info.clock).expect("JSON was not well-formatted");
        let str = clock.values().next().unwrap().to_string();
        let num: i32 = str.parse().unwrap();
        if num > max_clock {
            max_clock = num
        }
    }

    let mut clock: HashMap<String, i32> = HashMap::new();
    clock.insert(message.node_id, max_clock);

    let res = MessageDetailResponse {
        message_id: message.message_id.clone(),
        clock,
        from_addr: message.from.clone(),
        to_addr: message.to.clone(),
        raw_message: String::from_utf8_lossy(&message.data).to_string(),
        signature: String::from_utf8_lossy(&message.signature.unwrap()).to_string(),
    };
    Ok(Json(res))
}

async fn get_merge_log_by_message_id(Path(id): Path<String>) -> Result<Json<serde_json::Value>, StatusCode> {
    let conn = get_conn().await;

    let message: z_messages::Model;
    let query_res = z_messages::Entity::find().filter(z_messages::Column::MessageId.eq(id.clone())).one(conn).await.expect("Fail to query message");
    match query_res {
        Some(query_res) => {
            message = query_res;
        }
        None => return Err(StatusCode::NOT_FOUND)
    }

    let node_clock_info_query = clock_infos::Entity::find().filter(clock_infos::Column::MessageId.eq(id.clone())).one(conn).await.expect("Fail to query").unwrap();
    let start_merge_logs_query: Vec<merge_logs::Model> = merge_logs::Entity::find().filter(merge_logs::Column::SClockHash.eq(node_clock_info_query.clock_hash.clone())).all(conn).await.expect("Fail to query");
    let end_merge_logs_query: Vec<merge_logs::Model> = merge_logs::Entity::find().filter(merge_logs::Column::EClockHash.eq(node_clock_info_query.clock_hash.clone())).all(conn).await.expect("Fail to query");

    let result = serde_json::json!(
      {
          "start_merge_logs_query": start_merge_logs_query,
          "end_merge_logs_query": end_merge_logs_query,
      }
    );

    Ok(Json(result))
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let node = P2PNode {
        id: "406b4c9bb2117df0505a58c6c44a99c8817b7639d9c877bdbea5a8e4e0412740".parse().unwrap(),
        rpc_domain: ("127.0.0.1".to_string()),
        ws_domain: ("127.0.0.1".to_string()),
        rpc_port: 13333,
        ws_port: 13333,
        public_key: None,
    };
    for node in node.bfs_traverse().await {
        node.update_node_info().await;
        node.store_db().await;
    }

    // task::spawn(async move {
    //     let mut interval = time::interval(Duration::from_secs(10));
    //     loop {
    //         interval.tick().await;
    //         // let node = node_clone.lock().unwrap();
    //         for node in node.bfs_traverse().await {
    //             node.update_node_info().await;
    //             node.store_db().await;
    //         }
    //         println!("execute update node db task");
    //     }
    // });

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
                .route("/message/:id", get(get_message_by_id))
                .route("/merge_log_by_message_id/:id", get(get_merge_log_by_message_id))
                .layer(cors),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
