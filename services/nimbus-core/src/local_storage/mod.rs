use crate::{
    nimbus_object_service::nimbus_types::v1::{AccessType, ObjectType},
    storage::{NimbusObject, Storage},
};
use std::path::Path;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use tonic::{Request, Status, Streaming, metadata::MetadataMap};

use crate::nimbus_object_service::nimbus_public::v1::PutObjectRequest;

use std::result::Result;
use std::sync::Mutex;

#[derive(Debug, Default)]
pub struct LocalStorageCluster {
    files: Mutex<Vec<NimbusObject>>,
}

impl LocalStorageCluster {
    pub fn new() -> LocalStorageCluster {
        return LocalStorageCluster {
            files: Mutex::from(Vec::new()),
        };
    }
}

const LOCATION: &str = "local/";

// Extract the metadata from the metadata map.
fn get_meta_value(metadata: &MetadataMap, key: &str) -> String {
    metadata
        .get(key)
        .and_then(|val| val.to_str().ok())
        .unwrap()
        .to_string()
}

fn get_mata_enum_val(metadata: &MetadataMap, key: &str) -> Result<i32, Status> {
    let enum_id = metadata
        .get(key)
        .ok_or_else(|| Status::invalid_argument("Missing 'x-storage-class' header"))?
        .to_str()
        .map_err(|_| Status::invalid_argument("Invalid header character data"))?
        // 2. Parse the string integer back into the Protobuf Enum
        .parse::<i32>()
        .map_err(|_| Status::invalid_argument("x-storage-class must be a valid integer string"))?;

    Ok(enum_id)
}

#[tonic::async_trait]
impl Storage for LocalStorageCluster {
    async fn save(
        &self,
        request: Request<Streaming<PutObjectRequest>>, // Needs to be 'mut' to call stream.message().await
    ) -> Result<NimbusObject, Status> {
        // Using Status or your custom Error

        // Headers
        let metadata_map = request.metadata();

        let object_id = get_meta_value(metadata_map, "object_id");

        let access_type_enum_id = get_mata_enum_val(metadata_map, "access_type")?;

        // 3. Match against the generated Prost Enum variants
        let access_type = AccessType::try_from(access_type_enum_id)
            .map_err(|_| Status::invalid_argument("Invalid AccessType id."))?;

        let object_type_enum_id = get_mata_enum_val(metadata_map, "object_type")?;

        let object_type = ObjectType::try_from(object_type_enum_id)
            .map_err(|_| Status::invalid_argument("Invalid ObjectType id."))?;

        let content_type = get_meta_value(metadata_map, "content_type");

        let mut stream = request.into_inner();

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

        let mut object_size: usize = 0;
        // Iterate through the incoming gRPC stream chunks
        while let Some(message) = stream.message().await? {
            let data = message.data;
            object_size += data.len();
            file.write_all(&data)
                .await
                .map_err(|e| Status::internal(format!("Failed to write chunk: {}", e)))?;
        }

        // Ensure all data is fully pushed to disk
        file.flush()
            .await
            .map_err(|e| Status::internal(format!("Failed to flush file: {}", e)))?;

        let nimbus_object = NimbusObject::new(
            &object_id,
            object_size,
            access_type,
            object_type,
            &content_type,
        );

        self.files.lock().unwrap().push(nimbus_object.clone());

        Ok(nimbus_object)
    }
}
