use crate::nimbus_object_service::nimbus_proto::{PutObjectRequest, PutObjectResponse};
use std::result::Result;
use tonic::async_trait;
use tonic::{Status, Streaming};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save(
        &self,
        mut stream: Streaming<PutObjectRequest>,
        object_id: &str,
    ) -> Result<PutObjectResponse, Status>;
}
