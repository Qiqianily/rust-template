use tonic::{Request, Response, Status};

use crate::pb::greeter::{GreeterRequest, GreeterResponse, greeter_service_server::GreeterService};

// 实现 GreeterService trait
#[derive(Debug, Default)]
pub struct GreeterServiceImpl {
    // 这里面可以放数据库连接池
}

#[tonic::async_trait]
impl GreeterService for GreeterServiceImpl {
    async fn greeter(
        &self,
        request: Request<GreeterRequest>,
    ) -> Result<tonic::Response<GreeterResponse>, Status> {
        let name = request.into_inner().name;
        let response = GreeterResponse {
            message: format!("Hello, {}!", name),
        };
        Ok(Response::new(response))
    }
}
