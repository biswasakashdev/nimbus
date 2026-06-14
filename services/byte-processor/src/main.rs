use nimbus_object_service::{
    ByteProcessorService, nimbus_proto::nimbus_object_service_server::NimbusObjectServiceServer,
};

mod nimbus_object_service;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let byte_processor_service_impl = ByteProcessorService::default();

    println!("gRPC Server listening on {}", addr);

    Server::builder()
        .add_service(NimbusObjectServiceServer::new(byte_processor_service_impl))
        .serve(addr)
        .await?;

    Ok(())
}
