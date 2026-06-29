use crate::proto_gen::nimbus_cluster_lb::v1::{
    GetUrlRequest, GetUrlResponse, url_generator_service_server::UrlGeneratorService,
};

use std::{result::Result, str::FromStr};
use tonic::{Request, Response, Status, async_trait};

pub struct ClusterLB {
    address: String,
}

impl ClusterLB {
    pub fn new(addr: &str) -> ClusterLB {
        Self {
            address: String::from_str(addr).unwrap(),
        }
    }
}

#[async_trait]
impl UrlGeneratorService for ClusterLB {
    async fn get_url(
        &self,
        _request: Request<GetUrlRequest>,
    ) -> Result<Response<GetUrlResponse>, Status> {
        return Ok(Response::from(GetUrlResponse {
            url: self.address.clone(),
        }));
    }
}
