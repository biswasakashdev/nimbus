use crate::nimbus_object_service::nimbus_public::v1::PutObjectRequest;
use crate::nimbus_object_service::nimbus_types::v1::{AccessType, ObjectDetails, ObjectType};
use crate::storage::NimbusObjectAccessType::{PrivateAccess, PublicAccess};
use chrono::{DateTime, Utc};
use std::collections::hash_map;
use std::result::Result;
use std::str::FromStr;
use tonic::async_trait;
use tonic::{Request, Status, Streaming};

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save(
        &self,
        mut stream: Request<Streaming<PutObjectRequest>>,
    ) -> Result<NimbusObject, Status>;
}

#[derive(Debug, Clone)]
pub enum NimbusObjectType {
    Dir,
    File,
}

#[derive(Debug, Clone)]
pub enum NimbusObjectAccessType {
    PublicAccess,
    PrivateAccess,
}

#[derive(Debug, Clone)]
pub struct NimbusObject {
    object_id: String,
    object_size_bytes: usize,
    access_url: Option<String>,
    access_type: NimbusObjectAccessType,
    object_type: NimbusObjectType,
    created_on: DateTime<Utc>,
    content_type: String,
}

impl NimbusObject {
    pub fn new(
        object_id: &str,
        object_size: usize,
        acccess_type: AccessType,
        object_type: ObjectType,
        content_type: &str,
    ) -> NimbusObject {
        NimbusObject {
            object_id: String::from(object_id),
            object_size_bytes: object_size,
            access_url: None,
            access_type: match acccess_type {
                AccessType::Public => NimbusObjectAccessType::PublicAccess,
                _ => NimbusObjectAccessType::PrivateAccess,
            },
            object_type: match object_type {
                ObjectType::File => NimbusObjectType::File,
                _ => NimbusObjectType::Dir,
            },
            created_on: Utc::now(),
            content_type: String::from_str(content_type).unwrap(),
        }
    }

    pub fn to_object_details(&self) -> ObjectDetails {
        ObjectDetails {
            object_id: self.object_id.clone(),
            object_size: self.object_size_bytes as u64,
            access_type: match self.access_type {
                PrivateAccess => AccessType::Private.into(),
                PublicAccess => AccessType::Public.into(),
            },
            access_url: self.access_url.clone(),
            object_type: match self.object_type {
                NimbusObjectType::File => ObjectType::File.into(),
                NimbusObjectType::Dir => ObjectType::Directory.into(),
            },
            created_on: self.created_on.to_string(),
            meta_data: hash_map::HashMap::new(),
            content_type: self.content_type.clone(),
        }
    }
}
