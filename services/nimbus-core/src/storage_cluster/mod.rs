use scylla::client::session::Session;

pub struct StorageCluster {
    session: Session,
}

impl StorageCluster {
    pub fn new(session: Session) -> Self {
        Self { session: session }
    }
}
