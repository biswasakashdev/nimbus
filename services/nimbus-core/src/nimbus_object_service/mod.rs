use nimbus_proto::{
    DeleteObjectRequest, DeleteObjectResponse, FindObjectByIdRequest, FindObjectByIdResponse,
    GetDirectoryContentRequest, GetDirectoryContentResponse, GetObjectRequest, GetObjectResponse,
    PutObjectRequest, PutObjectResponse, UpdateAccessTypeRequest, UpdateAccessTypeResponse,
    nimbus_public_service_server::NimbusPublicService,
};
use std::{pin::Pin, result::Result};
use tokio::sync::mpsc;
use tokio_stream::{Stream, wrappers::ReceiverStream};
use tonic::{Request, Response, Status, Streaming};

use crate::storage::Storage;

pub mod nimbus_proto {
    include!("../proto-gen/nimbus_public/v1/nimbus_public.v1.rs");
}

#[derive(Default, Debug)]
pub struct NimbusCoreService<T> {
    storage: T,
}

impl<T: Storage> NimbusCoreService<T> {
    pub fn new(sn: T) -> NimbusCoreService<T> {
        NimbusCoreService { storage: sn }
    }
}

#[tonic::async_trait]
impl<T> NimbusPublicService for NimbusCoreService<T>
where
    T: Storage + Send + 'static,
{
    // Define the type signature for our response stream
    type GetObjectStream = Pin<Box<dyn Stream<Item = Result<GetObjectResponse, Status>> + Send>>;

    // Handler to create object and process the bytes.
    async fn put_object(
        &self,
        request: Request<Streaming<PutObjectRequest>>,
    ) -> Result<Response<PutObjectResponse>, Status> {
        let object_id = &request
            .metadata()
            .get("object_id")
            .and_then(|v| v.to_str().ok())
            .unwrap()
            .to_string();

        println!("{}", object_id);
        let stream = request.into_inner();

        let data = self.storage.save(stream, &object_id).await?;
        Ok(Response::new(data))
    }

    async fn get_object(
        &self,
        request: Request<GetObjectRequest>,
    ) -> Result<Response<Self::GetObjectStream>, Status> {
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

    async fn update_access_type(
        &self,
        _request: Request<UpdateAccessTypeRequest>,
    ) -> Result<Response<UpdateAccessTypeResponse>, Status> {
        let data = UpdateAccessTypeResponse::default();
        Ok(Response::new(data))
    }

    async fn get_directory_content(
        &self,
        _request: tonic::Request<GetDirectoryContentRequest>,
    ) -> std::result::Result<tonic::Response<GetDirectoryContentResponse>, tonic::Status> {
        let data = GetDirectoryContentResponse::default();
        Ok(Response::new(data))
    }
    async fn find_object_by_id(
        &self,
        _request: Request<FindObjectByIdRequest>,
    ) -> Result<Response<FindObjectByIdResponse>, tonic::Status> {
        let data = FindObjectByIdResponse::default();
        Ok(Response::from(data))
    }
}
