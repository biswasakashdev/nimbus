pub fn handler() -> String {
    String::from("Hello, I'm Akash Biswas")
}

mod byte_processor_gen {
    pub mod types {
        pub mod v1 {
            include!("../proto-gen/nimbus/v1/types/v1/nimbus.v1.types.v1.rs");
        }
    }

    include!("../proto-gen/nimbus/v1/nimbus.v1.rs");
}
