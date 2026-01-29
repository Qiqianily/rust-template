use template::pb::greeter::{GreeterRequest, greeter_service_client::GreeterServiceClient};
use tonic::Request;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = GreeterServiceClient::connect("http://[::1]:50051").await?;
    let request = Request::new(GreeterRequest {
        name: "Qiqianily".to_string(),
    });
    let response = client.greeter(request).await?;
    println!("response: {:?}", response);
    let r = response.into_inner();
    println!("✅ Greeter response:");
    println!("  Message: {}", r.message);
    Ok(())
}
