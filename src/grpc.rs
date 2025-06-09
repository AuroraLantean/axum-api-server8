use proto::admin_server::Admin;
use proto::calculator_server::Calculator;

//import compiled protobuf. The name must match the package name in your .proto file
pub mod proto {
  tonic::include_proto!("calculator");

  pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("calculator_descriptor");
}

pub type GrpcState = std::sync::Arc<tokio::sync::RwLock<u64>>;

#[derive(Debug, Default)]
pub struct AdminService {
  pub state: GrpcState,
}
#[tonic::async_trait]
impl Admin for AdminService {
  async fn get_count(
    &self,
    request: tonic::Request<proto::CountRequest>,
  ) -> Result<tonic::Response<proto::CountResponse>, tonic::Status> {
    println!("request: {:?}", request);
    let count = self.state.read().await;

    let response = proto::CountResponse { count: *count };
    Ok(tonic::Response::new(response))
  }
}

#[derive(Debug, Default)]
pub struct CalculatorService {
  pub(crate) state: GrpcState,
}
impl CalculatorService {
  async fn increment_counter(&self) {
    let mut count = self.state.write().await;
    *count += 1;
    println!("GrpcState count: {}", *count);
  }
}

#[tonic::async_trait]
impl Calculator for CalculatorService {
  async fn add(
    &self,
    request: tonic::Request<proto::CalculationRequest>,
  ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
    println!("request: {:?}", request);
    self.increment_counter().await;

    let input = request.get_ref();
    let response = proto::CalculationResponse {
      result: input.a + input.b,
    };
    Ok(tonic::Response::new(response))
  }
  async fn divide(
    &self,
    request: tonic::Request<proto::CalculationRequest>,
  ) -> Result<tonic::Response<proto::CalculationResponse>, tonic::Status> {
    println!("request: {:?}", request);
    self.increment_counter().await;

    let input = request.get_ref();
    if input.b == 0 {
      return Err(tonic::Status::invalid_argument("cannot divide by zero"));
    }; //see gRPC StatusCode...
    let response = proto::CalculationResponse {
      result: input.a / input.b,
    };
    Ok(tonic::Response::new(response))
  }
}
