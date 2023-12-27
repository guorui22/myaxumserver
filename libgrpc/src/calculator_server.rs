use tokio::sync::mpsc;
use tonic::{Request, Response, Status};
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;
use libproto::{Input, Output};
use libproto::calculator_service_server::CalculatorService;

pub struct Calculator;

#[tonic::async_trait]
impl CalculatorService for Calculator {
    /// unary
    async fn find_square(
        &self,
        request: tonic::Request<Input>,
    ) -> std::result::Result<tonic::Response<Output>, tonic::Status> {
        let input = request.into_inner();
        let output = Output {
            result: (input.number * input.number * 2) as i64,
        };
        Ok(tonic::Response::new(output))
    }

    type findFactorsStream = ReceiverStream<Result<Output, Status>>;
    async fn find_factors(&self, request: Request<Input>) -> Result<Response<Self::findFactorsStream>, Status> {
        let input = request.into_inner();

        let (tx, rx) = mpsc::channel(1000);
        tokio::spawn(async move {
            for i in 2..=input.number/2 {
                if input.number % i == 0 {
                    let output = Output {
                        result: i as i64,
                    };
                    tx.send(Ok(output)).await.unwrap();
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}