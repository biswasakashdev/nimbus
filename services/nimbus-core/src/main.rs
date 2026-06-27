pub mod local_storage;
pub mod nimbus_object_service;
pub mod storage;
pub mod storage_cluster;

use nimbus_object_service::{
    NimbusCoreService, nimbus_public::v1::nimbus_public_service_server::NimbusPublicServiceServer,
};

use scylla::client::session_builder::SessionBuilder;
use tonic::transport::Server;

use crate::local_storage::LocalStorageCluster;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
