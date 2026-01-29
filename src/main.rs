use template::{
    pb::greeter::greeter_service_server::GreeterServiceServer,
    service_impl::greeter::GreeterServiceImpl,
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse()?;
    let greeter_service = GreeterServiceImpl::default();

    println!("Starting UserService on {}", addr);

    Server::builder()
        .add_service(GreeterServiceServer::new(greeter_service))
        .serve(addr)
        .await?;

    Ok(())
}
