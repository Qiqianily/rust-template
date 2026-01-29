use template::{
    pb::{
        explanation::{
            GetExplanationByIdRequest, explanation_hu_service_client::ExplanationHuServiceClient,
        },
        greeter::{GreeterRequest, greeter_service_client::GreeterServiceClient},
    },
    service_impl::model::ExplanationHu,
};
use tonic::Request;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut greet_client = GreeterServiceClient::connect("http://[::1]:50001").await?;
    let greet_request = Request::new(GreeterRequest {
        name: "Qiqianily".to_string(),
    });
    let greet_response = greet_client.greeter(greet_request).await?;
    println!("greet_response: {:?}", greet_response);
    let r = greet_response.into_inner();
    println!("✅ Greeter response:");
    println!("  Message: {}", r.message);
    //
    let mut explanation_client = ExplanationHuServiceClient::connect("http://[::1]:50001").await?;
    let explanation_request = GetExplanationByIdRequest { id: 22 };
    let explanation_response = explanation_client
        .get_explanation_by_id(explanation_request)
        .await?;
    println!("response: {:?}", explanation_response);
    let r = explanation_response.into_inner();
    println!("✅ Explanation response:");
    let explanation_hu: ExplanationHu = r.into();
    println!("Answer: {}", serde_json::to_string_pretty(&explanation_hu)?);
    Ok(())
}
