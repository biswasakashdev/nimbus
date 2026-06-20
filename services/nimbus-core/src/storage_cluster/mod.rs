use scylla::client::{session::Session, session_builder::SessionBuilder};

use crate::storage::Storage;

pub struct StorageCluster {
    session: Session,
}

impl StorageCluster {
    fn new(session: Session) -> Self {
        Self { session: session }
    }
}
