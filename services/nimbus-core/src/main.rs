pub mod event_streamer;
pub mod local_storage;
pub mod nimbus_object_service;
pub mod proto_gen;
pub mod storage;
pub mod storage_cluster;

use proto_gen::{
    nimbus_public::v1::nimbus_public_service_server::NimbusPublicServiceServer,
    service_discovery::v1::nimbus_core_service_discovery_service_client::NimbusCoreServiceDiscoveryServiceClient,
};

use nimbus_object_service::NimbusCoreService;

use scylla::client::{self, session_builder::SessionBuilder};
use tonic::transport::Server;

use crate::local_storage::LocalStorageCluster;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async {});

    let addr = "[::1]:50052".parse()?;
    let scylla_url = "127.0.0.1:9042";
    let storage = NimbusCoreService::new(LocalStorageCluster::new());

    let _scylla_session = SessionBuilder::new().known_node(scylla_url).build().await?;

    println!("Connected to scylla DB.");

    println!("gRPC Server listening on {}", addr);

    Server::builder()
        .add_service(NimbusPublicServiceServer::new(storage))
        .serve(addr)
        .await?;

    Ok(())
}
