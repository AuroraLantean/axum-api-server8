use calculator::calculator_client::CalculatorClient;

use crate::calculator::CalculationRequest;

pub mod calculator {
  tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let server_addr = dotenvy::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());

  let addr = format!("http://{}", server_addr);
  let mut client = CalculatorClient::connect(addr).await?;

  let request = tonic::Request::new(CalculationRequest { a: 4, b: 5 });

  let response = client.add(request).await?;

  println!("RESPONSE={response:?}");

  Ok(())
}
