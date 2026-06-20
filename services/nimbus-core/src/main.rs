pub mod local_storage;
pub mod nimbus_object_service;
pub mod storage;
pub mod storage_cluster;

use nimbus_object_service::{
    NimbusCoreService, nimbus_proto::nimbus_public_service_server::NimbusPublicServiceServer,
};

use tonic::transport::Server;

use crate::local_storage::LocalStorageCluster;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse()?;
    let storage = NimbusCoreService::new(LocalStorageCluster::default());

    println!("gRPC Server listening on {}", addr);

    Server::builder()
        .add_service(NimbusPublicServiceServer::new(storage))
        .serve(addr)
        .await?;

    Ok(())
}
