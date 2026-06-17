use nimbus_object_service::{
    NimbusCoreService, nimbus_proto::nimbus_public_service_server::NimbusPublicServiceServer,
};

mod nimbus_object_service;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse()?;
    let byte_processor_service_impl = NimbusCoreService::default();

    println!("gRPC Server listening on {}", addr);

    Server::builder()
        .add_service(NimbusPublicServiceServer::new(byte_processor_service_impl))
        .serve(addr)
        .await?;

    Ok(())
}
