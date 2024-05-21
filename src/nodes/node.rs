use std::collections::{HashSet, VecDeque};
use reqwest::Client;
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use hex;
use prost::Message;
use crate::business;
use crate::vlc;
use crate::zmessage;

use sea_orm::{Database, DatabaseConnection, DbErr, EntityTrait, QuerySelect, TryFromU64};
use sea_orm::sea_query::Expr;
use sea_orm::entity::*;
use sea_orm::query::*;
use crate::db::get_conn;
use crate::entities::{z_messages, merge_logs, clock_infos, node_info};
use chrono::{NaiveDateTime, Utc, DateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct P2PNode {
    pub id: String,
    pub rpc_domain: String,
    pub ws_domain: String,
    pub rpc_port: u32,
    pub ws_port: u32,
    pub public_key: Option<String>,
}

impl P2PNode {
    pub async fn query_data(&self, gatewayType: u32, index: u32) -> Vec<u8> {
        let client = Client::new();
        let url = format!("http://{}:{}/rpc{}", self.rpc_domain, self.rpc_port, self.rpc_port);
        let request_data = json!({"method": "queryByKeyId","gatewayType":gatewayType,"index":index});
        let response = client
            .post(&url)
            .header("Content-Type", "application/json; charset=utf-8")
            .json(&request_data)
            .send()
            .await
            .expect("Failed to send request");


        if response.status().is_success() {
            let json_data: HashMap<String, Value> = response.json().await.expect("Failed to parse JSON");
            let hex_string = json_data["result"].as_str().expect("Expected a string");
            let mut bytes = vec![0u8; hex_string.len() / 2];
            hex::decode_to_slice(hex_string, &mut bytes).expect("Failed to decode hex string");
            let query_res = business::QueryResponse::decode(&*bytes);

            query_res.unwrap().data
        } else {
            Vec::new()
        }
    }

    pub async fn store_db(&self) {
        let conn = get_conn().await;

        let clock_nodes_max_id = get_clock_infos_max_id(conn).await.unwrap();
        let merge_logs_max_id = get_merge_logs_max_id(conn).await.unwrap();
        let z_messagers_max_id = get_z_messagers_max_id(conn).await.unwrap();

        let data = self.query_data(business::GatewayType::ClockNode as u32, clock_nodes_max_id).await;
        let clock_nodes = business::ClockNodes::decode(&*data).unwrap().clock_nodes;
        for x in clock_nodes {
            let clock_json = serde_json::to_string(&x.clock.unwrap().values).expect("Failed to serialize HashMap");
            let timestamp_secs = x.create_at / 1000;
            let timestamp_nanos = (x.create_at % 1000) * 1_000_000;
            let create_at = DateTime::from_timestamp(timestamp_secs as i64, timestamp_nanos as u32).unwrap().naive_utc();
            let new_clock_node = clock_infos::ActiveModel {
                id: NotSet,
                clock: ActiveValue::Set(clock_json),
                node_id: ActiveValue::Set(self.id.to_string()),
                message_id: ActiveValue::Set(String::from_utf8_lossy(&x.message_id).to_string()),
                raw_message: ActiveValue::Set(String::from_utf8_lossy(&x.raw_message).to_string()),
                event_count: ActiveValue::Set(x.count.try_into().unwrap()),
                create_at: ActiveValue::Set(Some(create_at)),
            };
            new_clock_node.insert(conn).await.expect("Fail to Insert Clock Node");
        }


        let data = self.query_data(business::GatewayType::MergeLog as u32, merge_logs_max_id).await;
        let merge_logs = vlc::MergeLogs::decode(&*data).unwrap().merge_logs;
        for x in merge_logs {
            let timestamp_secs = x.merge_at / 1000;
            let timestamp_nanos = (x.merge_at % 1000) * 1_000_000;
            let merge_at = DateTime::from_timestamp(timestamp_secs as i64, timestamp_nanos as u32).unwrap().naive_utc();

            let new_merge_log = merge_logs::ActiveModel {
                id: NotSet,
                from_id: ActiveValue::Set(String::from_utf8_lossy(&*x.from_id).to_string()),
                to_id: ActiveValue::Set(String::from_utf8_lossy(&x.to_id).to_string()),
                start_count: ActiveValue::Set(x.start_count.try_into().unwrap()),
                end_count: ActiveValue::Set(x.end_count.try_into().unwrap()),
                s_clock_hash: ActiveValue::Set(String::from_utf8_lossy(&x.s_clock_hash).to_string()),
                e_clock_hash: ActiveValue::Set(String::from_utf8_lossy(&x.e_clock_hash).to_string()),
                merge_at: ActiveValue::Set(merge_at),
                node_id: ActiveValue::Set(self.id.to_string()),
            };
            new_merge_log.insert(conn).await.expect("Fail to Insert Merge Log");
        }


        let index = 0;
        let gatewayType = 3;
        let data = self.query_data(business::GatewayType::ZMessage as u32, z_messagers_max_id).await;
        let zmessages = zmessage::ZMessages::decode(&*data).unwrap().messages;
        for x in zmessages {
            let version: Option<i32> = Some(x.version as i32);

            let new_message = z_messages::ActiveModel {
                id: NotSet,
                message_id: ActiveValue::Set(String::from_utf8_lossy(&x.id).to_string()),
                version: ActiveValue::Set(version),
                r#type: ActiveValue::Set(x.r#type.try_into().unwrap()),
                public_key: ActiveValue::Set(Option::from(String::from_utf8_lossy(&x.public_key).to_string())),
                data: ActiveValue::Set(x.data),
                signature: ActiveValue::Set(Option::from(x.signature)),
                from: ActiveValue::Set(String::from_utf8_lossy(&*x.from).to_string()),
                to: ActiveValue::Set(String::from_utf8_lossy(&*x.to).to_string()),
                node_id: ActiveValue::Set(self.id.to_string()),
            };
            new_message.insert(conn).await.expect("Fail to Insert Merge Log");
        }
    }

    pub async fn neighbors(&self) -> Vec<P2PNode> {
        let client = Client::new();

        let url = format!("http://{}:{}/rpc{}", self.rpc_domain, self.rpc_port, self.rpc_port);

        let request_data = json!({"method": "getNeighbors"});

        let response = client
            .post(&url)
            .header("Content-Type", "application/json; charset=utf-8")
            .json(&request_data)
            .send()
            .await
            .expect("Failed to send request");

        if response.status().is_success() {
            let json_data: HashMap<String, Value> = response.json().await.expect("Failed to parse JSON");
            let nodes: Vec<P2PNode> = json_data
                .into_iter()
                .map(|(k, v)| {
                    let rpc_port = v["rpcPort"].as_u64().unwrap() as u32;
                    let ws_port = v["wsPort"].as_u64().unwrap() as u32;
                    let node = P2PNode {
                        id: k,
                        rpc_domain: v["rpcDomain"].as_str().unwrap().to_string(),
                        ws_domain: v["wsDomain"].as_str().unwrap().to_string(),
                        rpc_port,
                        ws_port,
                        public_key: v["publicKey"].as_str().map(|s| s.to_string()),
                    };
                    node
                })
                .collect();

            nodes
        } else {
            Vec::new()
        }
    }

    pub async fn bfs_traverse(&self) -> Vec<P2PNode> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        queue.push_back(self.clone());
        visited.insert(self.id.clone());

        while let Some(current_node) = queue.pop_front() {
            result.push(current_node.clone());
            let neighbors = current_node.neighbors().await;
            for neighbor in neighbors {
                let neighbor_id = neighbor.id.clone();
                if !visited.contains(&neighbor_id) {
                    visited.insert(neighbor_id.clone());
                    queue.push_back(neighbor);
                }
            }
        }

        result
    }

    pub async fn update_node_info(&self) {
        let neighbors = self.neighbors().await;
        let conn = get_conn().await;
        let mut neighbor_nodes = Vec::new();
        for node  in neighbors {
            neighbor_nodes.push(node.id.parse().unwrap());
        }
        let mut is_alive = true;
        if neighbor_nodes.len() == 0 {
            is_alive = false;
        }

        if let Ok(Some(existing_record)) = node_info::Entity::find()
            .filter(node_info::Column::NodeId.eq(self.id.clone()))
            .one(conn)
            .await
        {
            let mut active_model: node_info::ActiveModel = existing_record.into();
            active_model.neighbor_nodes = Set(neighbor_nodes);
            active_model.is_alive = Set(is_alive);
            active_model.update(conn).await.expect("Fail to Update Node Info");
        } else {
            let new_node_info = node_info::ActiveModel {
                id: NotSet,
                node_id: Set(self.id.to_string()),
                neighbor_nodes: Set(neighbor_nodes),
                is_alive: Set(is_alive),
            };
            new_node_info.insert(conn).await.expect("Fail to Insert Node Info");
        }
    }
}


async fn get_merge_logs_max_id(db: &DatabaseConnection) -> Result<u32, DbErr> {
    let max_id = merge_logs::Entity::find()
        .select_only()
        .column_as(Expr::col(merge_logs::Column::Id).max(), "max_id")
        .into_tuple::<Option<i32>>()
        .one(db)
        .await?
        .flatten()
        .unwrap_or(0);
    Ok(max_id as u32)
}

async fn get_clock_infos_max_id(db: &DatabaseConnection) -> Result<u32, DbErr> {
    let max_id = clock_infos::Entity::find()
        .select_only()
        .column_as(Expr::col(clock_infos::Column::Id).max(), "max_id")
        .into_tuple::<Option<i32>>()
        .one(db)
        .await?
        .flatten()
        .unwrap_or(0);
    Ok(max_id as u32)
}

async fn get_z_messagers_max_id(db: &DatabaseConnection) -> Result<u32, DbErr> {
    let max_id = z_messages::Entity::find()
        .select_only()
        .column_as(Expr::col(z_messages::Column::Id).max(), "max_id")
        .into_tuple::<Option<i32>>()
        .one(db)
        .await?
        .flatten()
        .unwrap_or(0);
    Ok(max_id as u32)
}