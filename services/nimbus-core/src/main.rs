pub mod cluster_lb;
pub mod local_storage;
pub mod nimbus_object_service;
pub mod proto_gen;
pub mod storage;

use proto_gen::{
    nimbus_cluster_lb::v1::url_generator_service_server::UrlGeneratorServiceServer,
    nimbus_public::v1::nimbus_public_service_server::NimbusPublicServiceServer,
};

use nimbus_object_service::NimbusCoreService;

use tonic::transport::Server;

use crate::local_storage::LocalStorageCluster;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async {});

    let addr_str = "127.0.0.1:50051";

    let storage_service = NimbusCoreService::new(LocalStorageCluster::new());
    let lb_service = cluster_lb::ClusterLB::new(&addr_str);

    println!("gRPC Server listening on {}", addr_str);

    let sock_addr = addr_str.parse()?;

    Server::builder()
        .add_service(NimbusPublicServiceServer::new(storage_service))
        .add_service(UrlGeneratorServiceServer::new(lb_service))
        .serve(sock_addr)
        .await?;

    Ok(())
}
