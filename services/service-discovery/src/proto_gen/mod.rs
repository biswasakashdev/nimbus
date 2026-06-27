pub mod nimbus_cluster_lb {
    pub mod v1 {
        include!("../../out/nimbus_cluster_lb/v1/nimbus_cluster_lb.v1.rs");
    }
}
pub mod nimbus_types {
    pub mod v1 {
        include!("../../out/nimbus_types/v1/nimbus_types.v1.rs");
    }
}

pub mod service_discovery {
    pub mod v1 {
        include!("../../out/service_discovery/v1/service_discovery.v1.rs");
    }
}
