use crate::storage::Storage;
use std::path::Path;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use tonic::{Status, Streaming};

use crate::nimbus_object_service::nimbus_proto::{PutObjectRequest, PutObjectResponse};

use std::result::Result;

#[derive(Debug, Default)]
pub struct LocalStorageCluster {}

const LOCATION: &str = "local/";

#[tonic::async_trait]
impl Storage for LocalStorageCluster {
    // Changed 'self' to '&self' so you don't burn the struct instance
    async fn save(
        &self,
        mut stream: Streaming<PutObjectRequest>, // Needs to be 'mut' to call stream.message().await
        object_id: &str,
    ) -> Result<PutObjectResponse, Status> {
        // Using Status or your custom Error
        //
        let file_path = format!("{}{}", LOCATION, object_id);

        let path = Path::new(&file_path);

        // 1. Extract the parent directory path
        if let Some(parent) = path.parent() {
            // 2. Create the directories recursively if they don't exist
            fs::create_dir_all(parent)
                .await
                .map_err(|e| Status::internal(format!("Failed to create directories: {}", e)))?;
        }

        let mut file = File::create(path)
            .await
            .map_err(|e| Status::internal(format!("Failed to create file: {}", e)))?;

        // Iterate through the incoming gRPC stream chunks
        while let Some(message) = stream.message().await? {
            file.write_all(&message.data)
                .await
                .map_err(|e| Status::internal(format!("Failed to write chunk: {}", e)))?;
        }

        // Ensure all data is fully pushed to disk
        file.flush()
            .await
            .map_err(|e| Status::internal(format!("Failed to flush file: {}", e)))?;

        Ok(PutObjectResponse::default())
    }
}
