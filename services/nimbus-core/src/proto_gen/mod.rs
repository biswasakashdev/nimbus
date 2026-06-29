pub mod nimbus_types {
    pub mod v1 {
        include!("../../out/nimbus_types/v1/nimbus_types.v1.rs");
    }
}

pub mod nimbus_public {
    pub mod v1 {
        include!("../../out/nimbus_public/v1/nimbus_public.v1.rs");
    }
}

pub mod nimbus_cluster_lb {
    pub mod v1 {
        include!("../../out/nimbus_cluster_lb/v1/nimbus_cluster_lb.v1.rs");
    }
}
