use nimbus_proto::{
    CreateOrUpdateObjectRequest, CreateOrUpdateObjectResponse, DeleteObjectRequest,
    DeleteObjectResponse, GetObjectRequest, GetObjectResponse,
    nimbus_public_object_service_server::NimbusPublicObjectService,
};
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::{Stream, wrappers::ReceiverStream};
use tonic::{Request, Response, Status, Streaming};

pub mod nimbus_proto {
    pub mod types {
        pub mod v1 {
            include!("../proto-gen/nimbus_public/v1/types/v1/nimbus_public.v1.types.v1.rs");
        }
    }

    include!("../proto-gen/nimbus_public/v1/nimbus_public.v1.rs");
}
#[derive(Default, Debug)]
pub struct ByteProcessorService {}

#[tonic::async_trait]
impl NimbusPublicObjectService for ByteProcessorService {
    // Define the type signature for our response stream
    type GetObjectStream = Pin<Box<dyn Stream<Item = Result<GetObjectResponse, Status>> + Send>>;

    // Handler to create object and process the bytes.
    async fn create_or_update_object(
        &self,
        request: Request<Streaming<CreateOrUpdateObjectRequest>>,
    ) -> std::result::Result<Response<CreateOrUpdateObjectResponse>, Status> {
        let mut _stream = request.into_inner();
        let data = CreateOrUpdateObjectResponse::default();
        Ok(Response::from(data))
    }

    async fn get_object(
        &self,
        request: Request<GetObjectRequest>,
    ) -> Result<Response<Self::GetObjectStream>, tonic::Status> {
        let _id = request.into_inner().object_id;

        let (tx, rx) = mpsc::channel(10);

        tokio::spawn(async move {
            for i in 1..00 {
                let chunk = GetObjectResponse {
                    data: format!("Hello {i}").into_bytes(),
                };

                // Send the message to the channel. If the client disconnects,
                // tx.send() will error, and we break early to clean up resources.
                if tx.send(Ok(chunk)).await.is_err() {
                    eprintln!("Client disconnected early.");
                    break;
                }
            }
        });

        let output_stream = ReceiverStream::new(rx);

        Ok(Response::from(
            Box::pin(output_stream) as Self::GetObjectStream
        ))
    }

    async fn delete_object(
        &self,
        request: Request<DeleteObjectRequest>,
    ) -> std::result::Result<Response<DeleteObjectResponse>, Status> {
        let req: DeleteObjectRequest = request.into_inner();
        println!("{}", req.object_ids.len());
        Ok(Response::new(DeleteObjectResponse::default()))
    }
}
