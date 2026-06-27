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

pub mod service_discovery {
    pub mod v1 {
        include!("../../out/service_discovery/v1/service_discovery.v1.rs");
    }
}
