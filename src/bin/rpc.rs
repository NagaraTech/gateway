use tonic::transport::Server;

use gateway::grpc::gateway::gateway_server::GatewayServer;
use gateway::server::GatewayResponse;



mod store_proto {
    include!("../grpc/gateway.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("store_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let addr = "127.0.0.1:9001".parse()?;
    let gr = GatewayResponse{};


    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(store_proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(GatewayServer::new(gr))
        .add_service(reflection_service)
        .serve(addr)
        .await?;
    Ok(())
}

